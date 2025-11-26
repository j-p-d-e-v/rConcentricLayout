use crate::gpu::{GpuAdapter, GpuData, NodeConnectionsResult};
use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    CommandEncoderDescriptor, ComputePassDescriptor, ComputePipelineDescriptor,
    PipelineCompilationOptions, PipelineLayoutDescriptor, ShaderStages, include_wgsl,
    util::DeviceExt,
};

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct GpuNormalizeValue {
    pub node_id: u32,
    pub total: f32,
}

#[derive(Debug)]
pub struct BufferData {
    pub min_max_degree_buffer: Buffer,
    pub node_connections_buffer: Buffer,
    pub inner_result_buffer: Buffer,
    pub outer_result_buffer: Buffer,
}

#[derive(Debug)]
pub struct Normalize {
    pub adapter: GpuAdapter,
    pub gpu_data: GpuData,
    pub node_connections: NodeConnectionsResult,
}

impl Normalize {
    pub async fn new(
        gpu_data: &GpuData,
        node_connections: &NodeConnectionsResult,
    ) -> anyhow::Result<Self> {
        let adapter = GpuAdapter::new().await?;
        Ok(Self {
            adapter,
            gpu_data: gpu_data.clone(),
            node_connections: node_connections.to_owned(),
        })
    }

    pub async fn get_gpu_node_connections_data(&self) -> &Vec<[u32; 2]> {
        &self.node_connections.gpu_data
    }

    pub async fn get_buffer_data(&self) -> anyhow::Result<BufferData> {
        let device = &self.adapter.device;

        let min_max: &[u32; 2] = &[
            self.node_connections.data.min_degree,
            self.node_connections.data.max_degree,
        ];
        let node_connections_data = self.get_gpu_node_connections_data().await;
        let node_connections_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("normalize-node-connections"),
                contents: bytemuck::cast_slice(node_connections_data),
                usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
            });
        let min_max_degree_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("normalize-min-max-degree"),
            contents: bytemuck::cast_slice(min_max),
            usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        });
        let result_size = self.gpu_data.get_gpu_nodes_id().len() as u64
            * std::mem::size_of::<GpuNormalizeValue>() as u64;
        let inner_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("normalize-inner-result"),
            size: result_size,
            usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let outer_result_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("normalize-outer-result"),
            size: result_size,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        Ok(BufferData {
            min_max_degree_buffer,
            outer_result_buffer,
            inner_result_buffer,
            node_connections_buffer,
        })
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let device = &self.adapter.device;
        let buffer_data = self.get_buffer_data().await?;
        let data_bg_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("normalize-data-bg-layout"),
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
            label: Some("normalize-bg"),
            layout: &data_bg_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer_data.node_connections_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: buffer_data.min_max_degree_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: buffer_data.inner_result_buffer.as_entire_binding(),
                },
            ],
        });
        let data_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("normalize-data-pipeline-layout"),
            bind_group_layouts: &[&data_bg_layout],
            ..Default::default()
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("normalize-encoder"),
            ..Default::default()
        });
        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("normalize-compute-pipeline"),
            layout: Some(&data_pipeline_layout),
            module: &device.create_shader_module(include_wgsl!("wgsl/normalize.wgsl")),
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("normalize-compute-pass"),
                ..Default::default()
            });
            let num_dispatchers = (self.node_connections.gpu_data.len().div_ceil(64) + 10) as u32;
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

        self.adapter.queue.submit([encoder.finish()]);
        {
            let (tx, rx) = crossbeam::channel::bounded(1);

            buffer_data
                .outer_result_buffer
                .map_async(wgpu::MapMode::Read, .., move |result| {
                    tx.send(result).expect("normalize unable to send result")
                });

            device.poll(wgpu::wgt::PollType::wait_indefinitely())?;

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

            let buffer_result = buffer_data.outer_result_buffer.get_mapped_range(..);
            let data: &[GpuNormalizeValue] = bytemuck::cast_slice(&buffer_result);
            println!("{:#?}", data);
        }
        Ok(())
    }
}

#[cfg(test)]
pub mod test_gpu_normalize {
    use serde::Deserialize;

    use crate::{
        Edge, Node,
        gpu::{NodeConnectionsResult, data::GpuData, normalize::Normalize},
    };

    #[tokio::test]
    async fn test_normalize() {
        #[derive(Debug, Clone, Deserialize)]
        struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }
        let node_connections_reader = std::fs::File::options()
            .read(true)
            .open("storage/gpu-node-connections.json")
            .unwrap();
        let sample_data_reader = std::fs::File::options()
            .read(true)
            .open("storage/sample-data/sample-data.json")
            .unwrap();
        let node_connections_data =
            serde_json::from_reader::<_, NodeConnectionsResult>(node_connections_reader).unwrap();
        let sample_data = serde_json::from_reader::<_, SampleData>(sample_data_reader).unwrap();
        let gpu_data = GpuData::new(&sample_data.nodes, &sample_data.edges);
        let gpu_data = gpu_data.unwrap();
        let normalize = Normalize::new(&gpu_data, &node_connections_data).await;
        assert!(normalize.is_ok(), "{:?}", normalize.err());
        let normalize = normalize.unwrap();
        let result = normalize.execute().await;
        println!("{:#?}", result);
    }
}
