use std::collections::HashSet;

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
        let mut values: Vec<NodeConnectionValue> = Vec::new();
        let mut totals: HashSet<u32> = HashSet::new();
        for n in nodes {
            let total = edges
                .iter()
                .filter(|item| item.source == n.id || item.target == n.id)
                .count() as u32;
            totals.insert(total);
            values.push(NodeConnectionValue {
                node_id: n.id.clone(),
                total,
            });
        }
        values.sort_by(|a, b| b.total.cmp(&a.total));
        let max_degree = totals.iter().max().unwrap_or(&0).to_owned();
        let min_degree = totals.iter().min().unwrap_or(&0).to_owned();
        Ok(Self {
            max_degree,
            min_degree,
            values,
        })
    }
}
