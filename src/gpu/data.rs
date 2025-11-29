use anyhow::anyhow;
use bytemuck::{Pod, Zeroable};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Edge, Node};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub gpu_nodes: HashMap<u32, Node>,
    pub gpu_edges: Vec<GpuEdge>,
    pub gpu_nodes_id: Vec<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct GpuEdge {
    pub source_node: u32,
    pub target_node: u32,
}

impl GpuData {
    /// Transform the nodes into a flat Vec<u32> to make it much easier to load in the gpu.
    /// Transform the edges data into Vec<[u32;2]> to make it much easier to load in the gpu.
    pub fn new(nodes: &[Node], edges: &[Edge]) -> anyhow::Result<Self> {
        let mut gpu_edges: Vec<GpuEdge> = Vec::new();
        let gpu_nodes: HashMap<u32, Node> = nodes
            .iter()
            .enumerate()
            .map(|(index, item)| (index as u32, item.to_owned()))
            .collect();
        let gpu_nodes_id: Vec<u32> = gpu_nodes.keys().map(|index| index.to_owned()).collect();

        let edges_source_target: Vec<(Result<u32, anyhow::Error>, Result<u32, anyhow::Error>)> =
            edges
                .par_iter()
                .map(|item| {
                    let source = if let Some(value) = gpu_nodes
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
                    let target = if let Some(value) = gpu_nodes
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
        for (source, target) in edges_source_target {
            gpu_edges.push(GpuEdge {
                source_node: source?,
                target_node: target?,
            });
        }
        Ok(Self {
            nodes: nodes.to_vec(),
            edges: edges.to_vec(),
            gpu_edges,
            gpu_nodes,
            gpu_nodes_id,
        })
    }

    pub fn get_gpu_edges(&self) -> &Vec<GpuEdge> {
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
}

#[cfg(test)]
pub mod test_gpu_data {
    use super::*;
    use crate::{Edge, Node};
    use serde::Deserialize;

    #[tokio::test]
    async fn test_data() {
        #[derive(Debug, Clone, Deserialize)]
        struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }
        let sample_data_reader = std::fs::File::options()
            .read(true)
            .open("storage/sample-data/sample-data.json")
            .unwrap();
        let sample_data = serde_json::from_reader::<_, SampleData>(sample_data_reader).unwrap();
        assert!(!sample_data.nodes.is_empty());
        assert!(!sample_data.edges.is_empty());
        let gpu_data = GpuData::new(&sample_data.nodes, &sample_data.edges);
        assert!(gpu_data.is_ok(), "{:?}", gpu_data.err());
        let gpu_data = gpu_data.unwrap();
        assert!(!gpu_data.get_gpu_edges().is_empty());
        assert!(!gpu_data.get_gpu_nodes().is_empty());
        assert!(!gpu_data.get_gpu_nodes_id().is_empty());
    }
}
