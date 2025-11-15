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
    pub fn get(data: &NormalizeNodeConnections, total_rings: Option<u32>) -> anyhow::Result<Self> {
        let mut values: HashMap<u32, Vec<String>> = HashMap::new();
        let highest_normalized_value = data.max_value;
        let total_rings = if let Some(value) = total_rings {
            value
        } else {
            Self::default().total_rings
        };
        let last_ring_index = total_rings;
        for n in data.values.iter() {
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
        let mut index: u32 = 0;
        let mut values = values
            .iter()
            .map(|(_, nodes)| {
                let item = RingIndexValue {
                    index: index,
                    nodes: nodes.to_owned(),
                };
                index += 1;
                item
            })
            .collect::<Vec<RingIndexValue>>();
        values.sort_by(|a, b| a.index.cmp(&b.index));

        // If the highest degrees has multiple nodes. Elect one node as the center then push the others to the next ring.
        if let Some(item) = &values.first()
            && item.nodes.len() > 1
        {
            let original_index = item.index;
            let chosen_node = item.nodes[0].clone();
            values = values
                .iter_mut()
                .map(|i| {
                    if &i.index == &original_index {
                        i.nodes = i.nodes[1..].to_vec();
                    }
                    i.index += 1;
                    i.to_owned()
                })
                .collect::<Vec<RingIndexValue>>();
            values.push(RingIndexValue {
                index: original_index,
                nodes: vec![chosen_node],
            });
        }

        values.sort_by(|a, b| a.index.cmp(&b.index));
        let total_actual_rings = values.len();
        let max_nodes_per_ring = 36;
        if total_actual_rings > 1 {
            let mut index: usize = 1;
            loop {
                let (ring_index, nodes) = if let Some(item) = values.get(index) {
                    (item.index, item.nodes.to_owned())
                } else {
                    break;
                };
                let total_nodes = nodes.len();
                if total_nodes < max_nodes_per_ring {
                    index += 1;
                    continue;
                }
                values[index].nodes = nodes[0..max_nodes_per_ring].to_vec();
                let mut spill_nodes = nodes[max_nodes_per_ring..].to_vec();
                let next_index = index + 1;
                if let Some(next_item) = values.get_mut(next_index) {
                    next_item.nodes.append(&mut spill_nodes);
                } else {
                    let next_ring_index = ring_index + 1;
                    values.push(RingIndexValue {
                        index: next_ring_index,
                        nodes: spill_nodes,
                    });
                }
                index += 1;
            }
        }
        Ok(Self {
            total_rings,
            values,
        })
    }
}
