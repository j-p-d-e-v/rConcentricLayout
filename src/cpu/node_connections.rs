use crate::entities::{NodeConnectionValue, NodeConnectionsData};
use crate::{Edge, Node};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeConnections {}

impl NodeConnections {
    /// Get the connection count per node.
    /// Highest count will be the central node
    pub fn get(nodes: &Vec<Node>, edges: &Vec<Edge>) -> anyhow::Result<NodeConnectionsData> {
        let values: Vec<NodeConnectionValue> = nodes
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
        Ok(NodeConnectionsData::compute(values))
    }
}
