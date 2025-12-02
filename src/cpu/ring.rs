use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use serde::{Deserialize, Serialize};

use crate::entities::{NormalizeData, RingCapacity, RingData};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ring {}

impl Ring {
    // pub fn get_radius(radius: u32, ring_index: u32) -> u32 {
    //     radius * ring_index
    // }
    // pub fn get_max_nodes(radius: u32) -> usize {
    //     const L_MIN: f32 = 40_f32;
    //     (((2_f32 * PI) * radius as f32) / L_MIN).floor() as usize
    // }

    pub fn get(data: &NormalizeData) -> anyhow::Result<Vec<RingData>> {
        // let highest_normalized_value = data.max_value;
        // let step_radius: u32 = 40;
        let ring_capacity: Vec<RingCapacity> =
            RingCapacity::generate(data.values.len() as u32, Some(20));

        let mut values: Vec<RingData> = ring_capacity
            .par_iter()
            .map(|capacity| {
                let start = capacity.range[0] as usize;
                let end = capacity.range[1] as usize;
                let nodes: Vec<u32> = data
                    .values
                    .get(start..end)
                    .unwrap_or_default()
                    .par_iter()
                    .map(|item| item.node_id.to_owned())
                    .collect();
                RingData {
                    index: capacity.index,
                    nodes,
                    radius: capacity.radius,
                }
            })
            .collect();

        values.par_sort_by(|a, b| a.index.cmp(&b.index));
        Ok(values)
    }
}
