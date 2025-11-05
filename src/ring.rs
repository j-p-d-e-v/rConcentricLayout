use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::NormalizeNodeConnections;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingIndexes {
    pub total_rings: u32,
    pub values: Vec<RingIndexValue>,
}

impl Default for RingIndexes {
    fn default() -> Self {
        Self {
            total_rings: 4,
            values: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingIndexValue {
    pub index: u32,
    pub nodes: Vec<String>,
}

impl RingIndexes {
    /// Distribute each nodes to its respective rings
    /// Formula: ring_index = floor((HNV - NV) x R)
    /// R - total rings. Default: 4
    /// HNV - Highest normalized value
    /// NV - Node normalized value
    pub fn get(data: &NormalizeNodeConnections, total_rings: Option<u32>) -> Self {
        println!("{}", "=".repeat(10));
        let mut values: HashMap<u32, Vec<String>> = HashMap::new();
        let highest_normalized_value = data.max_value;
        let total_rings = if let Some(value) = total_rings {
            value
        } else {
            Self::default().total_rings
        };
        let last_ring_index = total_rings - 1;
        for n in &data.values {
            let mut ring_index = ((highest_normalized_value - n.normalized_value)
                * total_rings as f32)
                .floor() as u32;

            if ring_index > last_ring_index {
                ring_index = last_ring_index;
            }
            values
                .entry(ring_index)
                .and_modify(|item| item.push(n.node_id.clone()))
                .or_insert(vec![n.node_id.clone()]);
        }
        let mut values = values
            .iter()
            .map(|(index, nodes)| RingIndexValue {
                index: index.to_owned(),
                nodes: nodes.to_owned(),
            })
            .collect::<Vec<RingIndexValue>>();

        values.sort_by(|a, b| a.index.cmp(&b.index));
        Self {
            total_rings,
            values,
        }
    }
}
