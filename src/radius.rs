use crate::RingIndexes;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Radius {
    pub ring: u32,
    pub radius: u32,
}

impl Radius {
    /// Compute the radius for each rings
    /// Formula:
    /// radius = r_min + r_index + r_step
    /// - r_min = the minimum radius for each ring
    /// - r_index = the ring index / number
    /// - r_step = additional radius for each ring.
    pub fn get(
        ring_indexes: &RingIndexes,
        min_radius: Option<u32>,
        step_radius: Option<u32>,
    ) -> anyhow::Result<Vec<Self>> {
        let mut values: Vec<Self> = Vec::new();
        let mut r_step = 0;
        let mut counter: u32 = 1;
        for r in &ring_indexes.values {
            let r_min = if !values.is_empty() {
                min_radius.unwrap_or(40)
            } else {
                0
            };
            let radius = r_min + counter * r_step;
            values.push(Radius {
                ring: r.index,
                radius,
            });
            counter += 1;
            r_step += step_radius.unwrap_or(20);
        }
        Ok(values)
    }
}
