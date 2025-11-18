use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};

use crate::{Edge, Node};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeConnections {
    pub max_degree: u32,
    pub min_degree: u32,
    pub values: Vec<NodeConnectionValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnectionValue {
    pub node_id: String,
    pub total: u32,
}

impl NodeConnections {
    /// Get the connection count per node.
    /// Highest count will be the central node
    pub fn get(nodes: &Vec<Node>, edges: &Vec<Edge>) -> anyhow::Result<Self> {
        let mut values: Vec<NodeConnectionValue> = nodes
            .par_iter()
            .map(|node| {
                let total = edges
                    .par_iter()
                    .filter(|item| item.source == node.id || item.target == node.id)
                    .count() as u32;
                NodeConnectionValue {
                    node_id: node.id.clone(),
                    total,
                }
            })
            .collect::<Vec<NodeConnectionValue>>();
        values.sort_by(|a, b| b.total.cmp(&a.total));
        let totals = values
            .par_iter()
            .map(|item| item.total)
            .collect::<Vec<u32>>();
        let max_degree = totals.iter().max().unwrap_or(&0).to_owned();
        let min_degree = totals.iter().min().unwrap_or(&0).to_owned();
        Ok(Self {
            max_degree,
            min_degree,
            values,
        })
    }
}
