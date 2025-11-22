use crate::Edge;
use crate::Node;
use bytemuck::{Pod, Zeroable};
use serde::Deserialize;
use serde::Serialize;
use wgpu::Adapter;
use wgpu::Backends;
use wgpu::BindGroupDescriptor;
use wgpu::BindGroupLayout;
use wgpu::BindGroupLayoutDescriptor;
use wgpu::BindGroupLayoutEntry;
use wgpu::BufferUsages;
use wgpu::DeviceDescriptor;
use wgpu::Features;
use wgpu::Instance;
use wgpu::InstanceDescriptor;
use wgpu::InstanceFlags;
use wgpu::RequestAdapterOptions;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;

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
    let mut data = serde_json::from_reader::<_, SampleData>(reader).unwrap();

    let instance = Instance::new(&InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: Some("concentric gpu"),
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

    let nodes_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("nodes-data"),
        contents: bytemuck::cast_slice(&gpu_nodes),
        usage: BufferUsages::COPY_DST,
    });
    let edges_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("edges-data"),
        contents: bytemuck::cast_slice(&gpu_edges),
        usage: BufferUsages::COPY_DST,
    });

    //   device.create_bind_group_layout(&BindGroupLayoutDescriptor {
    //       label: Some("nodes-edges"),
    //       entries: &[
    //           BindGroupLayoutEntry {
    //
    //           }
    //       ]
    //   })
    //
    //   device.create_bind_group(&BindGroupDescriptor {
    //       label: Some("nodes-edges"),
    //       layout: BindGroupLayout::from(value)
    //   })
    println!("{:#?}", adapter.get_info());
    println!("Nodes: {} | Edges: {}", data.nodes.len(), data.edges.len());
}
