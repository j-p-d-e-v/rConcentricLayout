use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConnectionValue {
    pub node_id: String,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeConnectionsData {
    pub max_degree: u32,
    pub min_degree: u32,
    pub values: Vec<NodeConnectionValue>,
}

impl NodeConnectionsData {
    pub fn compute(values: Vec<NodeConnectionValue>) -> Self {
        let mut values = values;
        values.sort_by(|a, b| b.total.cmp(&a.total));
        let totals = values
            .par_iter()
            .map(|item| item.total)
            .collect::<Vec<u32>>();
        let max_degree = totals.par_iter().max().unwrap_or(&0).to_owned();
        let min_degree = totals.par_iter().min().unwrap_or(&0).to_owned();
        NodeConnectionsData {
            max_degree,
            min_degree,
            values,
        }
    }
}
