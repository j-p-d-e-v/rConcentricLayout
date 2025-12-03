use std::f32::consts::PI;

use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::entities::{NodePositionData, NormalizeData, RingCapacity};

#[derive(Debug)]
pub struct NodePositions {}

impl NodePositions {
    pub fn get(
        normalize_data: &NormalizeData,
        cx: Option<f32>,
        cy: Option<f32>,
    ) -> Vec<NodePositionData> {
        let ring_capacity: Vec<RingCapacity> =
            RingCapacity::generate(normalize_data.values.len() as u32, Some(20));
        let cx = cx.unwrap_or(0.0);
        let cy = cy.unwrap_or(0.0);
        let mut result = ring_capacity
            .par_iter()
            .fold(
                || {
                    let result: Vec<NodePositionData> = Vec::new();
                    result
                },
                |mut result, capacity| {
                    let start = capacity.range[0] as usize;
                    let end = capacity.range[1] as usize;
                    let nodes: Vec<u32> = normalize_data
                        .values
                        .get(start..end)
                        .unwrap_or_default()
                        .par_iter()
                        .map(|item| item.node_id.to_owned())
                        .collect();

                    let total_nodes = nodes.len();
                    let step_angle = 360_f32 / total_nodes as f32;
                    let mut items: Vec<NodePositionData> = nodes
                        .par_iter()
                        .enumerate()
                        .map(|(index, node_id)| {
                            let angle_degree = index as f32 * step_angle;
                            let angle_radian = angle_degree * (PI / 180_f32);
                            let ring_radius = capacity.radius;
                            let x = cx + ring_radius as f32 * angle_radian.cos();
                            let y = cy + ring_radius as f32 * angle_radian.sin();
                            NodePositionData {
                                index: capacity.index,
                                angle_degree,
                                angle_radian,
                                cx,
                                cy,
                                x,
                                y,
                                node_id: node_id.to_owned(),
                                radius: ring_radius,
                            }
                        })
                        .collect();
                    result.append(&mut items);
                    result
                },
            )
            .reduce(
                || {
                    let result: Vec<NodePositionData> = Vec::new();
                    result
                },
                |mut result, mut values| {
                    result.append(&mut values);
                    result
                },
            );
        result.par_sort_by(|a, b| a.index.cmp(&b.index));
        result
    }
}
