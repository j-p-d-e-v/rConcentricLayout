use serde::{Deserialize, Serialize};

use crate::NodeConnections;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NormalizeNodeConnections {
    pub max_value: f32,
    pub values: Vec<NormalizedValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedValue {
    pub node_id: String,
    pub degree: u32,
    pub max_degree: u32,
    pub min_degree: u32,
    pub normalized_value: f32,
}

impl NormalizeNodeConnections {
    /// Normalize the node connections
    /// Formula: normalized_value = (degree - min_degree) / (max_degree - min_degree)
    /// degree - is the number of edges per nodes. Refer to the connections per node count
    pub fn get(node_connections: &NodeConnections) -> Self {
        let mut values: Vec<NormalizedValue> = Vec::new();
        let max_degree = node_connections.max_degree;
        let min_degree = node_connections.min_degree;
        let mut max_value = 0.0;

        for item in &node_connections.values {
            let normalized_value =
                (item.total - min_degree) as f32 / (max_degree - min_degree) as f32;
            max_value = if normalized_value > max_value {
                normalized_value
            } else {
                max_value
            };
            values.push(NormalizedValue {
                node_id: item.node_id.clone(),
                degree: item.total,
                max_degree: max_degree,
                min_degree: min_degree,
                normalized_value,
            });
        }
        Self { max_value, values }
    }
}
