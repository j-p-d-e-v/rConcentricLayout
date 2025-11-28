use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator,
        ParallelIterator,
    },
    slice::ParallelSliceMut,
};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::entities::{NormalizeData, RingData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ring {}

impl Ring {
    pub fn get_radius(radius: u32, ring_index: u32) -> u32 {
        radius * ring_index
    }
    pub fn get_max_nodes(radius: u32) -> usize {
        const L_MIN: f32 = 40_f32;
        (((2_f32 * PI) * radius as f32) / L_MIN).floor() as usize
    }

    pub fn get(data: &NormalizeData) -> anyhow::Result<Vec<RingData>> {
        let highest_normalized_value = data.max_value;
        let step_radius: u32 = 40;

        // Assign Ring Index based on the normalized value.
        let mut values = data
            .values
            .par_iter()
            .fold(
                || {
                    let values: Vec<RingData> = Vec::new();
                    values
                },
                |mut values, item| {
                    let ring_index =
                        ((highest_normalized_value - item.value) * 2_f32).floor() as u32;
                    values.push(RingData {
                        index: 0,
                        radius: 0,
                        original_index: ring_index,
                        nodes: vec![item.node_id.to_owned()],
                    });
                    values
                },
            )
            .reduce(
                || {
                    let values: Vec<RingData> = Vec::new();
                    values
                },
                |mut values, items| {
                    for mut item in items {
                        let ring_index = item.original_index;
                        if let Some(value) = values
                            .iter_mut()
                            .find(|value| value.original_index == ring_index)
                        {
                            value.nodes.append(&mut item.nodes);
                        } else {
                            values.push(item);
                        }
                    }
                    values
                },
            )
            .par_iter_mut()
            .enumerate()
            .map(|(index, item)| {
                item.index = index as u32;
                item.to_owned()
            })
            .collect::<Vec<RingData>>();
        // Calculate Max Nodes per ring then if total nodes of the ring exceeds to the calculated max nodes, it will be moved to the next ring.
        // If the next ring exists, it will append to the existing next ring nodes but take note the appended node will be added at the top.
        // If the next ring does exists, it will create a new one.

        let mut previous_value: Option<RingData> = None;
        loop {
            values = values
                .par_iter()
                .fold(
                    || {
                        let values: Vec<RingData> = Vec::new();
                        values
                    },
                    |mut values, item| {
                        let mut item = item.to_owned();
                        let ring_index = item.index;
                        let nodes: Vec<String> = item.nodes.to_owned();
                        let min_radius = step_radius + (10 * ring_index);
                        let radius: u32 = if ring_index == 0 {
                            0
                        } else {
                            Self::get_radius(min_radius, ring_index)
                        };
                        let max_nodes_per_ring = if ring_index == 0 {
                            1
                        } else {
                            Self::get_max_nodes(radius)
                        };
                        if nodes.len() < max_nodes_per_ring {
                            item.radius = radius;
                            values.push(item.to_owned());
                        } else {
                            let non_spilled_nodes = if let Some(none_spilled_nodes) =
                                item.nodes.get(0..max_nodes_per_ring)
                            {
                                none_spilled_nodes.to_owned()
                            } else {
                                // Put a warning message here
                                Vec::new()
                            };
                            let spilled_nodes =
                                if let Some(spilled_nodes) = item.nodes.get(max_nodes_per_ring..) {
                                    spilled_nodes.to_owned()
                                } else {
                                    // Put a warning message here
                                    Vec::new()
                                };
                            if !non_spilled_nodes.is_empty() {
                                values.push(RingData {
                                    radius,
                                    nodes: non_spilled_nodes,
                                    ..item.to_owned()
                                });
                            }
                            if !spilled_nodes.is_empty() {
                                let ring_index = ring_index + 1;
                                let min_radius = step_radius + (10 * ring_index);
                                values.push(RingData {
                                    index: ring_index,
                                    nodes: spilled_nodes,
                                    radius: Self::get_radius(min_radius, ring_index),
                                    ..item.to_owned()
                                });
                            }
                        }
                        values
                    },
                )
                .reduce(
                    || {
                        let values: Vec<RingData> = Vec::new();
                        values
                    },
                    |mut values, items| {
                        for mut item_b in items {
                            if let Some(item_a) =
                                values.par_iter_mut().find_any(|b| b.index == item_b.index)
                            {
                                item_a.nodes.append(&mut item_b.nodes);
                            } else {
                                values.push(item_b);
                            }
                        }
                        values
                    },
                );
            if let Some(previous) = &previous_value
                && let Some(current) = values.last()
                && previous == current
            {
                break;
            } else {
                previous_value = values.last().map(|current| current.to_owned());
            }
        }
        values.par_sort_by(|a, b| a.index.cmp(&b.index));
        Ok(values)
    }
}
