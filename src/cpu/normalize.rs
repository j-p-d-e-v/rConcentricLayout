use crate::cpu::NodeConnections;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

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
    pub fn get(node_connections: &NodeConnections) -> anyhow::Result<Self> {
        let max_degree = node_connections.max_degree;
        let min_degree = node_connections.min_degree;

        let values: Vec<NormalizedValue> = node_connections
            .values
            .par_iter()
            .map(|item| {
                let item = item.to_owned();
                let normalized_value =
                    (item.total - min_degree) as f32 / (max_degree - min_degree) as f32;
                NormalizedValue {
                    node_id: item.node_id.clone(),
                    degree: item.total,
                    max_degree,
                    min_degree,
                    normalized_value,
                }
            })
            .collect::<Vec<NormalizedValue>>();

        let max_value = values
            .par_iter()
            .map(|item| item.normalized_value.to_owned())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less))
            .unwrap_or(0.0);

        Ok(Self { max_value, values })
    }
}
