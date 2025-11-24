use std::collections::HashMap;

use crate::Edge;
use crate::Node;
use bytemuck::{Pod, Zeroable};
use crossbeam::epoch::Pointable;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
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
use wgpu::PipelineCache;
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
    pub id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GpuEdge {
    pub id: u32,
    pub source: u32,
    pub target: u32,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct GpuConnections {
    pub node_id: u32,
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
        //        .open("storage/sample-data/sample-data.json")
        //.open("storage/sample-data/concentric_nonmesh_star_100.json")
        .open("storage/sample-data/graph_10000.json")
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
    let nodes_id: HashMap<u32, Node> = data
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (index as u32, node.to_owned()))
        .collect::<HashMap<u32, Node>>();
    let gpu_nodes: Vec<u32> = nodes_id
        .par_iter()
        .map(|item| *item.0)
        .collect::<Vec<u32>>();
    let gpu_edges: Vec<[u32; 2]> = data
        .edges
        .par_iter()
        .map(|item| {
            let source = nodes_id
                .par_iter()
                .find_any(|source| source.1.id == item.source)
                .unwrap()
                .0
                .to_owned();
            let target = nodes_id
                .par_iter()
                .find_any(|target| target.1.id == item.target)
                .unwrap()
                .0
                .to_owned();
            [source, target]
        })
        .collect::<Vec<[u32; 2]>>();

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
        size: ((size_of::<u32>() * 2) * gpu_nodes.len()) as u64,
        usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        mapped_at_creation: false,
    });
    let connections_result_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("connections-result-data"),
        size: ((size_of::<u32>() * 2) * gpu_nodes.len()) as u64,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
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

    let data_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("data-pipeline-layout"),
        bind_group_layouts: &[&data_bg_layout],
        push_constant_ranges: &[],
    });
    let connections_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
        label: Some("connections-pipeline"),
        layout: Some(&data_pipeline_layout),
        // layout: None,
        module: &device.create_shader_module(include_wgsl!("wgsl/connections.wgsl")),
        entry_point: Some("main"),
        compilation_options: PipelineCompilationOptions::default(),
        cache: Default::default(),
    });

    let data_bg_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("nodes-edges"),
        layout: &data_bg_layout,
        // layout: &connections_pipeline.get_bind_group_layout(0),
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
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("connections-encoder"),
    });
    {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("connections-compute-pass"),
            ..Default::default()
        });
        let total_dispatches = gpu_nodes.len().div_ceil(64) as u32;
        compute_pass.set_pipeline(&connections_pipeline);
        compute_pass.set_bind_group(0, &data_bg_group, &[]);
        compute_pass.dispatch_workgroups(total_dispatches, 1, 1);
    }
    encoder.copy_buffer_to_buffer(
        &connections_buffer,
        0,
        &connections_result_buffer,
        0,
        Some((size_of::<u32>() * 2 * gpu_nodes.len()) as u64),
    );
    queue.submit([encoder.finish()]);
    //    println!("{:#?}", adapter.get_info());
    println!("Nodes: {} | Edges: {}", gpu_nodes.len(), data.edges.len());
    //    println!("Nodes: {:?}", gpu_nodes);
    {
        let (tx, mut rx) = crossbeam::channel::bounded(1);

        connections_result_buffer.map_async(wgpu::MapMode::Read, .., move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        rx.recv().unwrap().unwrap();
        let result = connections_result_buffer.get_mapped_range(..);
        let data: &[[u32; 2]] = bytemuck::cast_slice(&result);
        println!("Resule Size: {}", data.len());
        println!("Result Data: {:?}", data);
    }
}
