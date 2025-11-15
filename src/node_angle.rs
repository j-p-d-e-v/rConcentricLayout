use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

use crate::RingIndexes;

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
    pub fn get(ring_indexes: &RingIndexes) -> anyhow::Result<Vec<NodeAngle>> {
        let mut values: Vec<NodeAngle> = Vec::new();
        for r in &ring_indexes.values {
            let nodes = &r.nodes;
            let total_nodes = nodes.len();
            let mut start_angle = 0_f32;
            let step_angle = (360_f32 / total_nodes as f32) as f32;

            for node in nodes.iter() {
                let angle_radian = start_angle * (PI / 180_f32);
                values.push(NodeAngle {
                    node: node.clone(),
                    angle_radian: angle_radian,
                    angle_degree: start_angle,
                    ring: r.index,
                });
                start_angle += step_angle;
            }
        }
        Ok(values)
    }
}
