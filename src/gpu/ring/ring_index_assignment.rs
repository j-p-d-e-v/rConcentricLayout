use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor,
    PipelineCompilationOptions, PipelineLayoutDescriptor, ShaderStages, include_wgsl,
    util::{BufferInitDescriptor, DeviceExt},
    wgt::PollType,
};

use crate::{
    entities::RingData,
    gpu::{GpuAdapter, GpuData, normalize::NormalizeResult},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct RingIndexAssignmentData {
    pub index: u32,
    pub node_id: u32,
}

#[derive(Debug)]
pub struct RingIndexAssignment {
    pub gpu_data: GpuData,
    pub adapter: GpuAdapter,
    pub normalize_result: NormalizeResult,
}

#[derive(Debug)]
pub struct BufferData {
    pub normalize_gpu_data_buffer: Buffer,
    pub normalize_max_value_buffer: Buffer,
    pub inner_result_buffer: Buffer,
    pub outer_result_buffer: Buffer,
}

impl RingIndexAssignment {
    pub async fn new(
        gpu_data: &GpuData,
        normalize_result: NormalizeResult,
    ) -> anyhow::Result<Self> {
        let adapter = GpuAdapter::new().await?;
        Ok(Self {
            adapter,
            normalize_result,
            gpu_data: gpu_data.clone(),
        })
    }

    pub async fn get_buffer_data(&self) -> BufferData {
        let device = &self.adapter.device;
        let result_size = (std::mem::size_of::<RingIndexAssignmentData>()
            * self.gpu_data.get_gpu_nodes_id().len()) as u64;
        let normalize_gpu_data_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ring-normalize-gpu-data"),
            contents: bytemuck::cast_slice(&self.normalize_result.gpu_data),
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        });
        let normalize_max_value_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("ring-normalize-max-value"),
            contents: &self.normalize_result.data.max_value.to_le_bytes(),
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
            normalize_max_value_buffer,
            inner_result_buffer,
            outer_result_buffer,
        }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
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
                    resource: buffer_data.normalize_max_value_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffer_data.inner_result_buffer.as_entire_binding(),
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ring-pipeline-layout"),
            bind_group_layouts: &[&data_bg_layout],
            ..Default::default()
        });
        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("ring-compute-pipeline"),
            layout: Some(&pipeline_layout),
            module: &device
                .create_shader_module(include_wgsl!("../wgsl/ring/ring_index_assignment.wgsl")),
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("ring-command-encoder"),
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("ring-compute-pass"),
                ..Default::default()
            });
            let num_dispatchers = self.normalize_result.gpu_data.len().div_ceil(64) as u32;
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &data_bg, &[]);
            compute_pass.dispatch_workgroups(num_dispatchers, 1, 1);
        }
        encoder.copy_buffer_to_buffer(
            &buffer_data.inner_result_buffer,
            0,
            &buffer_data.outer_result_buffer,
            0,
            buffer_data.inner_result_buffer.size(),
        );
        queue.submit([encoder.finish()]);
        {
            let (tx, rx) = crossbeam::channel::bounded(1);
            buffer_data
                .outer_result_buffer
                .map_async(wgpu::MapMode::Read, .., move |result| {
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

            let result_buffer = buffer_data.outer_result_buffer.get_mapped_range(..);
            let gpu_data: &[RingIndexAssignmentData] = bytemuck::cast_slice(&result_buffer);
            println!("{:#?}", gpu_data);
            println!("{}", gpu_data.len());
        }
        Ok(())
    }
}
