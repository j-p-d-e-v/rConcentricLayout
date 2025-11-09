use serde::{Deserialize, Serialize};

use crate::RingIndexes;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeAngle {
    pub node: String,
    pub angle_radian: f32,
    pub angle_degree: f32,
}

impl NodeAngle {
    /// Compute the angle for each nodes in a ring. This will contain both angle in radian and egree value
    /// Formulate: angle = start_angle + (2 * PI) * (k / total_nodes)
    /// k - node index in the ring.
    /// total_noeds - is Total Nodes per Ring
    pub fn get(
        ring_indexes: &RingIndexes,
        step_angle: Option<f32>,
    ) -> anyhow::Result<Vec<NodeAngle>> {
        let mut values: Vec<NodeAngle> = Vec::new();
        for r in &ring_indexes.values {
            let nodes = &r.nodes;
            let total_nodes = nodes.len();
            let mut start_angle = 0_f32;
            for (node_index, node) in nodes.iter().enumerate() {
                let new_start_angle = start_angle * (std::f32::consts::PI / 180_f32);
                let angle_radian = new_start_angle
                    + (2_f32 * std::f32::consts::PI) * (node_index as f32 / total_nodes as f32);
                let angle_degree = angle_radian * (180_f32 / std::f32::consts::PI);
                values.push(NodeAngle {
                    node: node.clone(),
                    angle_radian,
                    angle_degree,
                });
                start_angle += step_angle.unwrap_or(10.0);
            }
        }
        Ok(values)
    }
}
