use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RingData {
    pub index: u32, //Sequential Index
    pub nodes: Vec<u32>,
    pub radius: u32,
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct RingCapacity {
    pub index: u32,
    pub max_nodes: u32,
    pub radius: u32,
    pub range: [u32; 2],
}

impl RingCapacity {
    pub fn get_radius(radius: u32, ring_index: u32) -> u32 {
        radius * ring_index
    }

    pub fn get_max_nodes(radius: u32) -> u32 {
        const L_MIN: f32 = 40_f32;
        (((2_f32 * PI) * radius as f32) / L_MIN).floor() as u32
    }

    pub fn generate(total_nodes: u32, step_radius: Option<u32>) -> Vec<RingCapacity> {
        let mut total_max_nodes: u32 = 0;
        let mut ring_index: u32 = 0;
        let step_radius = step_radius.unwrap_or(10);
        let mut data: Vec<RingCapacity> = Vec::new();
        loop {
            if total_nodes < total_max_nodes as u32 {
                break;
            }
            let min_radius = if ring_index == 0 {
                0
            } else {
                step_radius + (10 * (ring_index + 1))
            };
            let start_index = total_max_nodes;
            let radius = if ring_index == 0 {
                0
            } else {
                Self::get_radius(min_radius, ring_index)
            };
            let max_nodes = if ring_index == 0 {
                1
            } else {
                Self::get_max_nodes(radius)
            };
            total_max_nodes += max_nodes;
            let end_index = if total_max_nodes as u32 > total_nodes {
                total_nodes
            } else {
                total_max_nodes
            };
            data.push(RingCapacity {
                index: ring_index,
                max_nodes,
                radius,
                range: [start_index, end_index],
            });
            ring_index += 1;
        }
        data
    }
}

#[cfg(test)]
pub mod test_ring_entity {
    use super::*;

    #[tokio::test]
    async fn test_ring_capacity() {
        let data = RingCapacity::generate(56, Some(10));
        assert!(!data.is_empty());
        assert!(data.iter().map(|item| item.max_nodes).sum::<u32>() >= 56);
        println!("{:#?}", data);
    }
}
