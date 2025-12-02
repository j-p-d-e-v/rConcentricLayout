use crate::{
    entities::{Edge, Node},
    gpu::GpuAdapter,
};
use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferUsages, BufferView, ComputePassDescriptor,
    ComputePipelineDescriptor, PipelineCompilationOptions, PipelineLayoutDescriptor, ShaderStages,
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::{BufferDescriptor, CommandEncoderDescriptor, PollType},
};

#[derive(Debug, Copy, Clone, Pod, Zeroable, Serialize, Deserialize)]
#[repr(C)]
pub struct GpuNodeConnectionValue {
    pub node_id: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeConnectionsResult {
    pub gpu_data: Vec<GpuNodeConnectionValue>,
    pub max_degree: u32,
    pub min_degree: u32,
}
#[derive(Debug)]
pub struct NodeConnections {
    pub adapter: GpuAdapter,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct BufferData {
    nodes_buffer: Buffer,
    edges_buffer: Buffer,
    inner_min_max_buffer: Buffer,
    inner_result_buffer: Buffer,
    outer_result_buffer: Buffer,
    outer_min_max_buffer: Buffer,
}

impl NodeConnections {
    pub async fn new(nodes: &Vec<Node>, edges: &Vec<Edge>) -> anyhow::Result<Self> {
        let adapter = GpuAdapter::new().await?;
        Ok(Self {
            adapter,
            nodes: nodes.to_owned(),
            edges: edges.to_owned(),
        })
    }

    pub async fn get_buffer_data(&self) -> BufferData {
        let device = &self.adapter.device;
        let nodes_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("node-connections-nodes-data"),
            contents: bytemuck::cast_slice(&self.nodes),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });
        let edges_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("node-connections-edges-data"),
            contents: bytemuck::cast_slice(&self.edges),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });
        let inner_min_max_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("node-connections-inner-min-max-data"),
            contents: bytemuck::cast_slice(&[0u32; 2]),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        });
        let result_size = (std::mem::size_of::<GpuNodeConnectionValue>() * self.nodes.len()) as u64;

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
        let outer_min_max_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("node-connections-outer-min-max-result"),
            size: inner_min_max_buffer.size(),
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        BufferData {
            nodes_buffer,
            edges_buffer,
            inner_min_max_buffer,
            inner_result_buffer,
            outer_result_buffer,
            outer_min_max_buffer,
        }
    }

    async fn get_buffer_view(&self, buffer_data: &Buffer) -> anyhow::Result<BufferView> {
        let device = &self.adapter.device;
        let (tx, rx) = crossbeam::channel::bounded(1);
        buffer_data.map_async(wgpu::MapMode::Read, .., move |result| {
            tx.send(result)
                .expect("unable to send node connection result")
        });
        device.poll(PollType::wait_indefinitely())?;

        match rx.recv() {
            Ok(received) => {
                if let Err(error) = received {
                    return Err(anyhow!(error.to_string()));
                }
            }
            Err(error) => {
                return Err(anyhow!(error.to_string()));
            }
        }
        let data = buffer_data.get_mapped_range(..);
        Ok(data)
    }

    pub async fn execute(&self) -> anyhow::Result<NodeConnectionsResult> {
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
                BindGroupLayoutEntry {
                    binding: 3,
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
                BindGroupEntry {
                    binding: 3,
                    resource: buffer_data.inner_min_max_buffer.as_entire_binding(),
                },
            ],
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("node-connections-encoder"),
        });
        for entry_point in &["get_connections", "get_min", "get_max"] {
            let compute_pipeline_label =
                format!("node-connections-{}-compute-pipeline", entry_point);
            let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some(&compute_pipeline_label),
                layout: Some(&data_pipeline_layout),
                // layout: None,
                module: &device.create_shader_module(include_wgsl!("wgsl/connections.wgsl")),
                entry_point: Some(&entry_point),
                compilation_options: PipelineCompilationOptions::default(),
                cache: Default::default(),
            });
            {
                let compute_pass_label = format!("node-connections-{}-compute-pass", entry_point);
                let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                    label: Some(&compute_pass_label),
                    ..Default::default()
                });
                let num_dispatchers = self.nodes.len().div_ceil(64) as u32 + 10;
                compute_pass.set_bind_group(0, &data_bg_group, &[]);
                compute_pass.set_pipeline(&compute_pipeline);
                compute_pass.dispatch_workgroups(num_dispatchers, 1, 1);
            }
        }
        encoder.copy_buffer_to_buffer(
            &buffer_data.inner_result_buffer,
            0,
            &buffer_data.outer_result_buffer,
            0,
            buffer_data.inner_result_buffer.size(),
        );
        encoder.copy_buffer_to_buffer(
            &buffer_data.inner_min_max_buffer,
            0,
            &buffer_data.outer_min_max_buffer,
            0,
            buffer_data.inner_min_max_buffer.size(),
        );
        self.adapter.queue.submit([encoder.finish()]);
        let result: NodeConnectionsResult = {
            let buffered_data = self
                .get_buffer_view(&buffer_data.outer_result_buffer)
                .await?;
            let gpu_data: &[GpuNodeConnectionValue] = bytemuck::cast_slice(&buffered_data);

            let buffered_data = self
                .get_buffer_view(&buffer_data.outer_min_max_buffer)
                .await?;
            let min_max: &[u32] = bytemuck::cast_slice(&buffered_data);
            NodeConnectionsResult {
                gpu_data: gpu_data.to_vec(),
                max_degree: min_max[1],
                min_degree: min_max[0],
            }
        };
        buffer_data.outer_result_buffer.unmap();
        Ok(result)
    }
}

#[cfg(test)]
pub mod test_gpu_node_connections {
    use super::*;
    use crate::gpu::node_connections::NodeConnections;
    use serde::Deserialize;

    #[tokio::test]
    async fn test_node_connections() {
        #[derive(Debug, Clone, Deserialize)]
        struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }

        let reader = std::fs::File::options()
            .read(true)
            .open("storage/sample-data/nodes_100_full_mesh.json")
            .unwrap();
        let sample_data = serde_json::from_reader::<_, SampleData>(reader).unwrap();
        let node_connections = NodeConnections::new(&sample_data.nodes, &sample_data.edges).await;
        assert!(node_connections.is_ok(), "{:?}", node_connections.err());
        let node_connections = node_connections.unwrap();
        let result = node_connections.execute().await;
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        println!("Nodes: {}", result.gpu_data.len());
        let mut writer = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open("storage/gpu-node-connections.json")
            .unwrap();
        serde_json::to_writer_pretty(&mut writer, &result).unwrap();
    }
}
