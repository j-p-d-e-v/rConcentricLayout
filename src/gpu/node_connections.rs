use crate::{
    entities::{NodeConnectionValue, NodeConnectionsData},
    gpu::{GpuAdapter, GpuData},
};
use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferUsages, ComputePassDescriptor,
    ComputePipelineDescriptor, PipelineCompilationOptions, PipelineLayoutDescriptor, ShaderStages,
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::{BufferDescriptor, CommandEncoderDescriptor},
};

#[derive(Debug, Copy, Clone, Pod, Zeroable, Serialize, Deserialize)]
#[repr(C)]
pub struct GpuNodeConnectionValue {
    pub node_id: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnectionsResult {
    pub gpu_data: Vec<GpuNodeConnectionValue>,
    pub data: NodeConnectionsData,
}
#[derive(Debug)]
pub struct NodeConnections {
    pub adapter: GpuAdapter,
    pub gpu_data: GpuData,
}

#[derive(Debug)]
pub struct BufferData {
    nodes_buffer: Buffer,
    edges_buffer: Buffer,
    inner_result_buffer: Buffer,
    outer_result_buffer: Buffer,
}

impl NodeConnections {
    pub async fn new(gpu_data: &GpuData) -> anyhow::Result<Self> {
        let adapter = GpuAdapter::new().await?;
        Ok(Self {
            adapter,
            gpu_data: gpu_data.to_owned(),
        })
    }

    pub async fn get_buffer_data(&self) -> BufferData {
        let device = &self.adapter.device;
        let nodes_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("node-connections-nodes-data"),
            contents: bytemuck::cast_slice(self.gpu_data.get_gpu_nodes_id()),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });
        let edges_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("node-connections-edges-data"),
            contents: bytemuck::cast_slice(self.gpu_data.get_gpu_edges()),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });
        let result_size = (std::mem::size_of::<GpuNodeConnectionValue>()
            * self.gpu_data.get_gpu_nodes_id().len()) as u64;

        let inner_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("node-connections-innert-result"),
            size: result_size,
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let outer_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("node-connections-outer-result"),
            size: result_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        BufferData {
            nodes_buffer,
            edges_buffer,
            inner_result_buffer,
            outer_result_buffer,
        }
    }

    pub async fn run(&self) -> anyhow::Result<NodeConnectionsResult> {
        let buffer_data = self.get_buffer_data().await;
        let device = &self.adapter.device;
        let data_bg_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("node-connections-data-bg-layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let data_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("node-connections-data-pipeline-layout"),
            bind_group_layouts: &[&data_bg_layout],
            push_constant_ranges: &[],
        });
        let data_bg_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("nodes-connections-bg-group"),
            layout: &data_bg_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer_data.nodes_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffer_data.edges_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffer_data.inner_result_buffer.as_entire_binding(),
                },
            ],
        });
        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("node-connections-compute-pipeline"),
            layout: Some(&data_pipeline_layout),
            // layout: None,
            module: &device.create_shader_module(include_wgsl!("wgsl/connections.wgsl")),
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: Default::default(),
        });

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("node-connections-encoder"),
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("node-connections-compute-pass"),
                ..Default::default()
            });
            let num_dispatchers = self.gpu_data.gpu_nodes_id.len().div_ceil(64) as u32 + 10;
            compute_pass.set_bind_group(0, &data_bg_group, &[]);
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.dispatch_workgroups(num_dispatchers, 1, 1);
        }
        encoder.copy_buffer_to_buffer(
            &buffer_data.inner_result_buffer,
            0,
            &buffer_data.outer_result_buffer,
            0,
            buffer_data.inner_result_buffer.size(),
        );
        self.adapter.queue.submit([encoder.finish()]);
        let result: NodeConnectionsResult = {
            let (tx, rx) = crossbeam::channel::bounded(1);
            buffer_data
                .outer_result_buffer
                .map_async(wgpu::MapMode::Read, .., move |result| {
                    tx.send(result)
                        .expect("node connections unable to send result");
                });

            device.poll(wgpu::wgt::PollType::wait_indefinitely())?;
            match rx.recv() {
                Ok(data) => {
                    if let Err(error) = data {
                        return Err(anyhow!(error.to_string()));
                    }
                }
                Err(error) => return Err(anyhow!(error.to_string())),
            };

            let buffered_data = buffer_data.outer_result_buffer.get_mapped_range(..);
            let gpu_data: &[GpuNodeConnectionValue] = bytemuck::cast_slice(&buffered_data);
            let mut values: Vec<NodeConnectionValue> = Vec::new();
            let mapped_values = gpu_data
                .par_iter()
                .map(
                    |item| match self.gpu_data.get_gpu_nodes().get(&item.node_id) {
                        Some(node) => Ok(NodeConnectionValue {
                            node_id: node.id.to_owned(),
                            total: item.total,
                        }),
                        None => Err(anyhow!(format!(
                            "unable to find node for index {:#?}",
                            item
                        ))),
                    },
                )
                .collect::<Vec<Result<NodeConnectionValue, anyhow::Error>>>();
            for value in mapped_values {
                values.push(value?);
            }
            NodeConnectionsResult {
                gpu_data: gpu_data.to_vec(),
                data: NodeConnectionsData::compute(values),
            }
        };
        buffer_data.outer_result_buffer.unmap();
        Ok(result)
    }
}

#[cfg(test)]
pub mod test_gpu_node_connections {
    use serde::Deserialize;

    use crate::{
        Edge, Node,
        gpu::{data::GpuData, node_connections::NodeConnections},
    };

    #[tokio::test]
    async fn test_node_connections() {
        #[derive(Debug, Clone, Deserialize)]
        struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }

        let reader = std::fs::File::options()
            .read(true)
            .open("storage/sample-data/sample-data.json")
            .unwrap();
        let sample_data = serde_json::from_reader::<_, SampleData>(reader).unwrap();
        let gpu_data = GpuData::new(&sample_data.nodes, &sample_data.edges);
        let gpu_data = gpu_data.unwrap();
        let node_connections = NodeConnections::new(&gpu_data).await;
        assert!(node_connections.is_ok(), "{:?}", node_connections.err());
        let node_connections = node_connections.unwrap();
        let result = node_connections.run().await;
        assert!(result.is_ok(), "{:?}", result.err());

        let result = result.unwrap();
        let mut writer = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open("storage/gpu-node-connections.json")
            .unwrap();
        serde_json::to_writer_pretty(&mut writer, &result).unwrap();
    }
}
