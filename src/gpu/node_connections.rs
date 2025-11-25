use crate::{
    Edge, Node,
    entities::{NodeConnectionValue, NodeConnectionsData},
    gpu::GpuAdapter,
};
use anyhow::{Error, anyhow};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferUsages, ComputePassDescriptor,
    ComputePipelineDescriptor, PipelineCompilationOptions, PipelineLayoutDescriptor, ShaderStages,
    include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::{BufferDescriptor, CommandEncoderDescriptor},
};

#[derive(Debug)]
pub struct NodeConnections {
    pub adapter: GpuAdapter,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    gpu_nodes: HashMap<u32, Node>,
    gpu_edges: Vec<[u32; 2]>,
    gpu_nodes_id: Vec<u32>,
}

#[derive(Debug)]
pub struct BufferData {
    nodes_buffer: Buffer,
    edges_buffer: Buffer,
    inner_result_buffer: Buffer,
    outer_result_buffer: Buffer,
}

impl NodeConnections {
    pub async fn new(nodes: &[Node], edges: &[Edge]) -> anyhow::Result<Self> {
        let adapter = GpuAdapter::new().await?;
        Ok(Self {
            adapter,
            gpu_nodes: HashMap::new(),
            gpu_edges: Vec::new(),
            gpu_nodes_id: Vec::new(),
            nodes: nodes.to_owned(),
            edges: edges.to_owned(),
        })
    }

    pub fn get_gpu_edges(&self) -> &Vec<[u32; 2]> {
        &self.gpu_edges
    }

    pub fn get_gpu_nodes(&self) -> &HashMap<u32, Node> {
        &self.gpu_nodes
    }

    pub fn get_gpu_nodes_bytes_size(&self) -> u64 {
        let size = std::mem::size_of::<u32>() * self.gpu_nodes.len();
        size as u64
    }

    pub fn get_gpu_nodes_id(&self) -> &Vec<u32> {
        &self.gpu_nodes_id
    }

    /// Transform the nodes into a flat Vec<u32> to make it much easier to load in the gpu.
    /// Transform the edges data into Vec<[u32;2]> to make it much easier to load in the gpu.
    pub async fn prepare_data(&mut self) -> anyhow::Result<()> {
        let mapping: HashMap<u32, Node> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(index, item)| (index as u32, item.to_owned()))
            .collect();
        let indexes: Vec<u32> = mapping.keys().map(|index| index.to_owned()).collect();
        self.gpu_nodes = mapping;
        self.gpu_nodes_id = indexes;

        let edges: Vec<(Result<u32, Error>, Result<u32, Error>)> = self
            .edges
            .par_iter()
            .map(|item| {
                let source = if let Some(value) = self
                    .gpu_nodes
                    .par_iter()
                    .find_any(|node| node.1.id == item.source)
                {
                    Ok(value.0.to_owned())
                } else {
                    Err(anyhow!(format!(
                        "gpu_connections_edge_source_not_found: {:?}",
                        item
                    )))
                };
                let target = if let Some(value) = self
                    .gpu_nodes
                    .par_iter()
                    .find_any(|node| node.1.id == item.target)
                {
                    Ok(value.0.to_owned())
                } else {
                    Err(anyhow!(format!(
                        "gpu_connections_edge_target_not_found: {:?}",
                        item
                    )))
                };
                (source, target)
            })
            .collect();
        for (source, target) in edges {
            self.gpu_edges.push([source?, target?]);
        }
        Ok(())
    }

    pub async fn get_buffer_data(&self) -> BufferData {
        let device = &self.adapter.device;
        let nodes_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("node-connections-nodes-data"),
            contents: bytemuck::cast_slice(self.get_gpu_nodes_id()),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });
        let edges_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("node-connections-edges-data"),
            contents: bytemuck::cast_slice(self.get_gpu_edges()),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });
        let inner_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("node-connections-innert-result"),
            size: self.get_gpu_nodes_bytes_size() * 2,
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let outer_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("node-connections-outer-result"),
            size: self.get_gpu_nodes_bytes_size() * 2,
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

    pub async fn run(&self) -> anyhow::Result<NodeConnectionsData> {
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
            let num_dispatchers = self.gpu_nodes_id.len().div_ceil(64) as u32 + 10;
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
        let values: Vec<NodeConnectionValue> = {
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
            let data: &[[u32; 2]] = bytemuck::cast_slice(&buffered_data);
            data.par_iter()
                .map(|item| {
                    let node_index_id = item[0];
                    let total = item[1];
                    let node = self
                        .get_gpu_nodes()
                        .get(&node_index_id)
                        .expect("node_index_id does not exists in gpu_nodes mapping");
                    NodeConnectionValue {
                        node_id: node.id.to_owned(),
                        total,
                    }
                })
                .collect()
        };
        buffer_data.outer_result_buffer.unmap();
        Ok(NodeConnectionsData::compute(values))
    }
}

#[cfg(test)]
pub mod test_gpu_node_connections {
    use serde::Deserialize;

    use crate::{Edge, Node, gpu::node_connections::NodeConnections};

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
        assert!(!sample_data.nodes.is_empty());
        assert!(!sample_data.edges.is_empty());
        let node_connections = NodeConnections::new(&sample_data.nodes, &sample_data.edges).await;
        assert!(node_connections.is_ok(), "{:?}", node_connections.err());
        let mut node_connections = node_connections.unwrap();
        let prepared_data = node_connections.prepare_data().await;
        assert!(prepared_data.is_ok(), "{:?}", prepared_data.err());
        assert!(!node_connections.get_gpu_edges().is_empty());
        assert!(!node_connections.get_gpu_nodes().is_empty());
        assert!(!node_connections.get_gpu_nodes_id().is_empty());
        let result = node_connections.run().await;
        assert!(result.is_ok(), "{:?}", result.err());
        println!("Result: {:#?}", result.unwrap());
    }
}
