use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, BufferView,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor,
    PipelineCompilationOptions, PipelineLayoutDescriptor, ShaderStages, include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::PollType,
};

use crate::{
    entities::{Edge, Node, NodePositionData, RingCapacity},
    gpu::{GpuAdapter, normalize::NormalizeResult},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodePositionsResult {
    pub gpu_data: Vec<NodePositionData>,
}
#[derive(Debug)]
pub struct NodePositions {
    pub adapter: GpuAdapter,
    pub normalize_result: NormalizeResult,
    pub ring_capacity: Vec<RingCapacity>,
    pub cx: f32,
    pub cy: f32,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct BufferData {
    pub normalize_gpu_data_buffer: Buffer,
    pub ring_capacity_buffer: Buffer,
    pub inner_result_buffer: Buffer,
    pub cx_cy_buffer: Buffer,
    pub outer_result_buffer: Buffer,
}

impl NodePositions {
    pub async fn new(
        nodes: &Vec<Node>,
        edges: &Vec<Edge>,
        normalize_result: NormalizeResult,
        cx: Option<f32>,
        cy: Option<f32>,
    ) -> anyhow::Result<Self> {
        let adapter = GpuAdapter::new().await?;
        let ring_capacity = RingCapacity::generate(nodes.len() as u32, Some(20));
        Ok(Self {
            adapter,
            ring_capacity,
            normalize_result,
            cx: cx.unwrap_or(0.0),
            cy: cy.unwrap_or(0.0),
            nodes: nodes.to_owned(),
            edges: edges.to_owned(),
        })
    }

    pub async fn get_buffer_data(&self) -> BufferData {
        let device = &self.adapter.device;
        let total_nodes = self.nodes.len();
        let result_size = (std::mem::size_of::<NodePositionData>() * total_nodes) as u64;
        let normalize_gpu_data_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ring-normalize-gpu-data"),
            contents: bytemuck::cast_slice(&self.normalize_result.gpu_data),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        });
        let ring_capacity_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ring-ring-capacity-data"),
            contents: bytemuck::cast_slice(&self.ring_capacity),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        });
        let cx_cy_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ring-cx-cy-data"),
            contents: bytemuck::cast_slice(&[self.cx, self.cy]),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        });
        let inner_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("ring-inner-result"),
            size: result_size,
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let outer_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("ring-outer-result"),
            size: result_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        BufferData {
            normalize_gpu_data_buffer,
            cx_cy_buffer,
            ring_capacity_buffer,
            inner_result_buffer,
            outer_result_buffer,
        }
    }

    async fn get_buffer_view(&self, buffer_data: &Buffer) -> anyhow::Result<BufferView> {
        let device = &self.adapter.device;
        let (tx, rx) = crossbeam::channel::bounded(1);
        buffer_data.map_async(wgpu::MapMode::Read, .., move |result| {
            tx.send(result).expect("unable to send ring result")
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

    pub async fn execute(&self) -> anyhow::Result<NodePositionsResult> {
        let device = &self.adapter.device;
        let queue = &self.adapter.queue;
        let buffer_data = self.get_buffer_data().await;
        let data_bg_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("ring-data-bg-layout"),
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
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                //Inner Result
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
        let data_bg = device.create_bind_group(&BindGroupDescriptor {
            label: Some("ring-data-bg"),
            layout: &data_bg_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer_data.normalize_gpu_data_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffer_data.ring_capacity_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffer_data.cx_cy_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: buffer_data.inner_result_buffer.as_entire_binding(),
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ring-pipeline-layout"),
            bind_group_layouts: &[&data_bg_layout],
            ..Default::default()
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("ring-command-encoder"),
        });
        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("ring-compute-pipeline"),
            layout: Some(&pipeline_layout),
            module: &device.create_shader_module(include_wgsl!("wgsl/positions.wgsl")),
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("ring-compute-pass"),
                ..Default::default()
            });
            let num_dispatchers = self.ring_capacity.len().div_ceil(64) as u32;
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &data_bg, &[]);
            compute_pass.dispatch_workgroups(num_dispatchers, 1, 1);
        }
        encoder.copy_buffer_to_buffer(
            &buffer_data.inner_result_buffer,
            0,
            &buffer_data.outer_result_buffer,
            0,
            buffer_data.outer_result_buffer.size(),
        );
        queue.submit([encoder.finish()]);
        let result = {
            let outer_result_buffer = self
                .get_buffer_view(&buffer_data.outer_result_buffer)
                .await?;
            let gpu_data: &[NodePositionData] = bytemuck::cast_slice(&outer_result_buffer);
            NodePositionsResult {
                gpu_data: gpu_data.to_vec(),
            }
        };
        buffer_data.outer_result_buffer.unmap();
        Ok(result)
    }
}

#[cfg(test)]
pub mod test_gpu_node_positions {
    use super::*;
    use serde::Deserialize;

    #[tokio::test]
    async fn test_node_positions() {
        #[derive(Debug, Clone, Deserialize)]
        struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }
        let normalize_reader = std::fs::File::options()
            .read(true)
            .open("storage/gpu-normalize.json")
            .unwrap();
        let sample_data_reader = std::fs::File::options()
            .read(true)
            .open("storage/sample-data/nodes_100_full_mesh.json")
            .unwrap();
        let normalize_data =
            serde_json::from_reader::<_, NormalizeResult>(normalize_reader).unwrap();
        let sample_data = serde_json::from_reader::<_, SampleData>(sample_data_reader).unwrap();
        let positions = NodePositions::new(
            &sample_data.nodes,
            &sample_data.edges,
            normalize_data,
            None,
            None,
        )
        .await;
        assert!(positions.is_ok(), "{:?}", positions.err());
        let positions = positions.unwrap();
        let result = positions.execute().await;
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        let mut writer = std::fs::File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open("storage/gpu-node-positions.json")
            .unwrap();
        serde_json::to_writer_pretty(&mut writer, &result).unwrap();
    }
}
