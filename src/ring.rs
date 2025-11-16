use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::NormalizeNodeConnections;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingIndexes {
    pub values: Vec<RingIndexValue>,
}

impl Default for RingIndexes {
    fn default() -> Self {
        Self { values: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingIndexValue {
    pub index: u32,
    pub original_index: u32,
    pub nodes: Vec<String>,
    pub radius: u32,
}

impl RingIndexes {
    pub fn get_radius(radius: u32, ring_index: u32) -> u32 {
        let new_radius = radius * ring_index;
        new_radius
    }
    pub fn get_max_nodes(radius: u32) -> usize {
        const L_MIN: f32 = 40_f32;
        let result = (((2_f32 * PI) * radius as f32) / L_MIN).floor() as usize;
        result
    }

    /// Distribute each nodes to its respective rings
    /// Formula: ring_index = floor((HNV - NV) x R)
    /// R - total rings. Default: 4
    /// HNV - Highest normalized value
    /// NV - Node normalized value
    pub fn get(data: &NormalizeNodeConnections) -> anyhow::Result<Self> {
        let highest_normalized_value = data.max_value;
        let last_ring_index = 2;
        let mut values: Vec<RingIndexValue> = Vec::new();
        let mut index = 0;
        for n in data.values.iter() {
            let mut ring_index =
                ((highest_normalized_value - n.normalized_value) * 2_f32).floor() as u32;
            if ring_index > last_ring_index {
                ring_index = last_ring_index;
            }
            if let Some(item) = values
                .iter_mut()
                .find(|item| item.original_index == ring_index)
            {
                item.nodes.push(n.node_id.to_owned());
            } else {
                values.push(RingIndexValue {
                    index,
                    radius: 0,
                    original_index: ring_index,
                    nodes: vec![n.node_id.to_owned()],
                });
                index += 1;
            }
        }
        let mut index: usize = 0;
        let step_radius: u32 = 10;
        let mut min_radius = 30;
        loop {
            let item = if let Some(item) = values.get(index) {
                item.to_owned()
            } else {
                break;
            };
            let ring_index = item.index;
            let nodes: Vec<String> = item.nodes;
            let radius: u32 = if index == 0 {
                0
            } else {
                Self::get_radius(min_radius, ring_index)
            };
            let max_nodes_per_ring = if index == 0 {
                1
            } else {
                Self::get_max_nodes(radius)
            };
            let total_nodes = nodes.len();
            values[index].radius = radius;
            if total_nodes < max_nodes_per_ring {
                index += 1;
                continue;
            }
            min_radius += step_radius;
            values[index].nodes = nodes[0..max_nodes_per_ring].to_vec();
            index += 1;
            let spill_nodes = nodes[max_nodes_per_ring..].to_vec();
            if let Some(next_item) = values.get_mut(index) {
                let mut nodes = spill_nodes;
                nodes.append(&mut next_item.nodes);
                next_item.nodes = nodes;
            } else {
                values.push(RingIndexValue {
                    index: index as u32,
                    nodes: spill_nodes,
                    radius: Self::get_radius(min_radius, index as u32),
                    original_index: item.original_index,
                });
            }
        }
        Ok(Self { values })
    }
}
