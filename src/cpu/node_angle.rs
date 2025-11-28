use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::entities::RingData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeAngle {
    pub node: String,
    pub angle_radian: f32,
    pub angle_degree: f32,
    pub ring: u32,
}

impl NodeAngle {
    /// Compute the angle for each nodes in a ring. This will contain both angle in radian and egree value
    /// Step Angle - The incrementor of the angle. Formula: 360 / total nodes
    /// Radian - The position in radian. Formula: Ange * (PI / 180)
    pub fn get(rings: &Vec<RingData>) -> anyhow::Result<Vec<NodeAngle>> {
        //        let mut values: Vec<NodeAngle> = Vec::new();
        let mut values: Vec<NodeAngle> = rings
            .par_iter()
            .fold(
                || {
                    let values: Vec<NodeAngle> = Vec::new();
                    values
                },
                |mut values, r| {
                    let nodes = &r.nodes;
                    let total_nodes = nodes.len();
                    let step_angle = 360_f32 / total_nodes as f32;
                    let mut result = nodes
                        .par_iter()
                        .enumerate()
                        .map(|(index, node)| {
                            let angle_degree = index as f32 * step_angle;
                            let angle_radian = angle_degree * (PI / 180_f32);
                            NodeAngle {
                                node: node.clone(),
                                angle_radian,
                                angle_degree,
                                ring: r.index,
                            }
                        })
                        .collect::<Vec<NodeAngle>>();
                    values.append(&mut result);
                    values
                },
            )
            .reduce(
                || {
                    let values: Vec<NodeAngle> = Vec::new();
                    values
                },
                |mut values, mut items| {
                    values.append(&mut items);
                    values
                },
            );
        values.par_sort_by(|a, b| a.ring.cmp(&b.ring));
        Ok(values)
    }
}
