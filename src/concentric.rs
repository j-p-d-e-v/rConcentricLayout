use crate::{Edge, Node, NodeConnections, NormalizeNodeConnections};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concentric {
    pub node_connections: NodeConnections,
    pub normalized_values: NormalizeNodeConnections,
}

impl Concentric {
    pub fn new() -> Self {
        Self {
            node_connections: NodeConnections::default(),
            normalized_values: NormalizeNodeConnections::default(),
        }
    }

    /// Count the number of edges/paths per node
    pub fn count_node_connections(&mut self, nodes: &Vec<Node>, edges: &Vec<Edge>) {
        self.node_connections = NodeConnections::get(nodes, edges)
    }

    /// Normalize Node Connections
    pub fn normalize_node_connections(&mut self) {
        self.normalized_values = NormalizeNodeConnections::get(&self.node_connections);
    }
}
