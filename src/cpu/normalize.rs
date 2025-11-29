use crate::entities::NodeConnectionsData;
use crate::entities::{NormalizeData, NormalizeValue};
use anyhow::anyhow;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Normalize {}

impl Normalize {
    /// Normalize the node connections
    /// Formula: normalized_value = (degree - min_degree) / (max_degree - min_degree)
    /// degree - is the number of edges per nodes. Refer to the connections per node count
    pub fn get(node_connections: &NodeConnectionsData) -> anyhow::Result<NormalizeData> {
        let max_degree = node_connections.max_degree;
        let min_degree = node_connections.min_degree;

        let mut values: Vec<NormalizeValue> = node_connections
            .values
            .par_iter()
            .map(|item| {
                let item = item.to_owned();
                let normalized_value =
                    (item.total - min_degree) as f32 / (max_degree - min_degree) as f32;
                NormalizeValue {
                    node_id: item.node_id.clone(),
                    value: if normalized_value.is_nan() {
                        0.0
                    } else {
                        normalized_value
                    },
                }
            })
            .collect::<Vec<NormalizeValue>>();

        values.par_sort_by(|a, b| {
            b.value
                .partial_cmp(&a.value)
                .expect("unable to sort normalize value")
        });
        let max_value = match std::panic::catch_unwind(|| {
            values
                .par_iter()
                .map(|item| item.value.to_owned())
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        }) {
            Ok(value) => value,
            Err(_) => {
                return Err(anyhow!("unable to find max value at normalize"));
            }
        };

        Ok(NormalizeData { max_value, values })
    }
}
