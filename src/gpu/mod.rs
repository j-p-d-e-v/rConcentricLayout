use crate::Edge;
use crate::Node;
use bytemuck::{Pod, Zeroable};
use serde::Deserialize;
use serde::Serialize;
use wgpu::Adapter;
use wgpu::Backends;
use wgpu::BindGroupDescriptor;
use wgpu::BindGroupEntry;
use wgpu::BindGroupLayout;
use wgpu::BindGroupLayoutDescriptor;
use wgpu::BindGroupLayoutEntry;
use wgpu::BindingResource;
use wgpu::BindingType;
use wgpu::BufferAddress;
use wgpu::BufferBinding;
use wgpu::BufferBindingType;
use wgpu::BufferUsages;
use wgpu::CommandBuffer;
use wgpu::ComputePassDescriptor;
use wgpu::ComputePipelineDescriptor;
use wgpu::DeviceDescriptor;
use wgpu::Features;
use wgpu::FeaturesWGPU;
use wgpu::FeaturesWebGPU;
use wgpu::Instance;
use wgpu::InstanceDescriptor;
use wgpu::InstanceFlags;
use wgpu::PipelineCompilationOptions;
use wgpu::PipelineLayoutDescriptor;
use wgpu::RequestAdapterOptions;
use wgpu::ShaderModule;
use wgpu::ShaderStages;
use wgpu::include_wgsl;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;
use wgpu::wgt::BufferDescriptor;
use wgpu::wgt::CommandEncoderDescriptor;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GpuNode {
    pub id: [u8; 32],
    pub label: [u8; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GpuEdge {
    pub id: [u8; 32],
    pub source: [u8; 32],
    pub target: [u8; 32],
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GpuConnections {
    pub node_id: [u8; 32],
    pub total: u32,
}

#[tokio::test]
pub async fn test_wgpu() {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct SampleData {
        nodes: Vec<Node>,
        edges: Vec<Edge>,
    }
    let reader = std::fs::File::options()
        .read(true)
        .open("storage/sample-data/sample-data.json")
        .unwrap();
    let data = serde_json::from_reader::<_, SampleData>(reader).unwrap();

    let instance = Instance::new(&InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: Some("concentric-gpu"),
            ..Default::default()
        })
        .await
        .unwrap();

    let gpu_nodes = data
        .nodes
        .iter()
        .map(|item| {
            let mut id = [0u8; 32];
            let i_id = item.id.as_bytes();
            id[..i_id.len()].copy_from_slice(&i_id[..i_id.len()]);
            let mut label = [0u8; 32];
            let i_label = item.label.as_bytes();
            label[..i_label.len()].copy_from_slice(&i_label[..i_label.len()]);
            GpuNode { id, label }
        })
        .collect::<Vec<GpuNode>>();

    let gpu_edges = data
        .edges
        .iter()
        .map(|item| {
            let mut id = [0u8; 32];
            let i_id = item.id.as_bytes();
            id[..i_id.len()].copy_from_slice(&i_id[..i_id.len()]);
            let mut source = [0u8; 32];
            let i_source = item.source.as_bytes();
            source[..i_source.len()].copy_from_slice(&i_source[..i_source.len()]);
            let mut target = [0u8; 32];
            let i_target = item.target.as_bytes();
            target[..i_target.len()].copy_from_slice(&i_target[..i_target.len()]);
            GpuEdge { id, source, target }
        })
        .collect::<Vec<GpuEdge>>();
    let gpu_connections: Vec<GpuConnections> = Vec::new();

    let nodes_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("nodes-data"),
        contents: bytemuck::cast_slice(&gpu_nodes),
        usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
    });
    let edges_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("edges-data"),
        contents: bytemuck::cast_slice(&gpu_edges),
        usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
    });
    let connections_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("connections-data"),
        size: gpu_nodes.len() as u64,
        usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let data_bg_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("data-bg-layout"),
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

    let data_bg_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("nodes-edges"),
        layout: &data_bg_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: nodes_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: edges_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 2,
                resource: connections_buffer.as_entire_binding(),
            },
        ],
    });
    let data_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("data-pipeline-layout"),
        bind_group_layouts: &[&data_bg_layout],
        push_constant_ranges: &[],
    });
    let connections_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("connections-pipeline"),
        layout: Some(&data_pipeline_layout),
        module: &device.create_shader_module(include_wgsl!("wgsl/connections.wgsl")),
        entry_point: Some("main"),
        compilation_options: PipelineCompilationOptions::default(),
        cache: None,
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("connections-encoder"),
    });
    {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("connections-compute-pass"),
            ..Default::default()
        });
        compute_pass.set_bind_group(0, &data_bg_group, &[]);
        compute_pass.set_pipeline(&connections_pipeline);
        compute_pass.dispatch_workgroups(64, 1, 1);
    }

    queue.submit([encoder.finish()]);
    println!("{:#?}", adapter.get_info());
    println!("Nodes: {} | Edges: {}", data.nodes.len(), data.edges.len());
}
