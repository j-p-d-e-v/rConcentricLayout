use rayon::{
    iter::{
        IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator,
        ParallelIterator,
    },
    slice::ParallelSliceMut,
};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::NormalizeNodeConnections;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ring {
    pub index: u32, //Sequential Index
    pub original_index: u32,
    pub nodes: Vec<String>,
    pub radius: u32,
}

impl Ring {
    pub fn get_radius(radius: u32, ring_index: u32) -> u32 {
        let new_radius = radius * ring_index;
        new_radius
    }
    pub fn get_max_nodes(radius: u32) -> usize {
        const L_MIN: f32 = 40_f32;
        let result = (((2_f32 * PI) * radius as f32) / L_MIN).floor() as usize;
        result
    }

    pub fn get(data: &NormalizeNodeConnections) -> anyhow::Result<Vec<Self>> {
        let highest_normalized_value = data.max_value;
        let step_radius: u32 = 20;
        let mut min_radius = 30;

        // Assign Ring Index based on the normalized value.
        let mut values = data
            .values
            .par_iter()
            .fold(
                || {
                    let values: Vec<Ring> = Vec::new();
                    values
                },
                |mut values, item| {
                    let ring_index =
                        ((highest_normalized_value - item.normalized_value) * 2_f32).floor() as u32;
                    let radius = Self::get_radius(min_radius, ring_index);
                    values.push(Ring {
                        index: 0,
                        radius,
                        original_index: ring_index,
                        nodes: vec![item.node_id.to_owned()],
                    });
                    values
                },
            )
            .reduce(
                || {
                    let values: Vec<Ring> = Vec::new();
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
                item.radius = if index == 0 { 0 } else { item.radius };
                item.to_owned()
            })
            .collect::<Vec<Ring>>();

        let mut index: usize = 0;
        // Calculate Max Nodes per ring then if total nodes of the ring exceeds to the calculated max nodes, it will be moved to the next ring.
        // If the next ring exists, it will append to the existing next ring nodes but take note the appended node will be added at the top.
        // If the next ring does exists, it will create a new one.
        let mut values = values
            .par_iter()
            .enumerate()
            .fold(
                || {
                    let values: Vec<Ring> = Vec::new();
                    values
                },
                |mut values, (index, item)| {
                    let ring_index = item.index;
                    let item = if let Some(vitem) = values
                        .par_iter_mut()
                        .find_any(|vitem| vitem.index == ring_index)
                    {
                        let mut item = item.to_owned();
                        let mut nodes: Vec<String> = Vec::new();
                        nodes.append(&mut vitem.nodes);
                        nodes.append(&mut item.nodes);
                        item.nodes = nodes;
                        item.radius = Self::get_radius(step_radius, ring_index * 2);
                        item
                    } else {
                        item.to_owned()
                    };
                    let nodes: Vec<String> = item.nodes.to_owned();
                    let max_nodes_per_ring = if ring_index == 0 {
                        1
                    } else {
                        Self::get_max_nodes(item.radius)
                    };
                    if nodes.len() < max_nodes_per_ring {
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
                            values.push(Ring {
                                nodes: non_spilled_nodes,
                                radius: Self::get_radius(step_radius, ring_index * 2),
                                ..item.clone()
                            });
                        }
                        if !spilled_nodes.is_empty() {
                            values.push(Ring {
                                nodes: spilled_nodes,
                                index: ring_index + 1,
                                radius: Self::get_radius(step_radius, (ring_index + 1) * 2),
                                ..item
                            });
                        }
                    }
                    values
                },
            )
            .reduce(
                || {
                    let values: Vec<Ring> = Vec::new();
                    values
                },
                |mut values, mut items| {
                    values.append(&mut items);
                    values
                },
            );
        values.par_sort_by(|a, b| a.index.cmp(&b.index));
        println!("{:#?}", values);
        //  loop {
        //      let item = if let Some(item) = values.get(index) {
        //          item.to_owned()
        //      } else {
        //          break;
        //      };
        //      let nodes: Vec<String> = item.nodes;
        //      let max_nodes_per_ring = if index == 0 {
        //          1
        //      } else {
        //          Self::get_max_nodes(item.radius)
        //      };
        //      if nodes.len() < max_nodes_per_ring {
        //          index += 1;
        //          continue;
        //      }
        //      if let Some(current_item) = values.get_mut(index)
        //          && let Some(left_nodes) = nodes.get(0..max_nodes_per_ring)
        //      {
        //          current_item.nodes = left_nodes.to_owned();
        //      }
        //      min_radius += step_radius;
        //      index += 1;
        //      let spill_nodes = if let Some(spill_nodes) = nodes.get(max_nodes_per_ring..) {
        //          spill_nodes.to_owned()
        //      } else {
        //          continue;
        //      };
        //      if let Some(next_item) = values.get_mut(index) {
        //          let mut nodes = spill_nodes;
        //          nodes.append(&mut next_item.nodes);
        //          next_item.nodes = nodes;
        //          next_item.radius = Self::get_radius(min_radius, index as u32);
        //      } else {
        //          values.push(Ring {
        //              index: index as u32,
        //              nodes: spill_nodes,
        //              radius: Self::get_radius(min_radius, index as u32),
        //              original_index: item.original_index,
        //          });
        //      }
        //  }
        Ok(values)
    }
}
