use serde::{Deserialize, Serialize};

use crate::{NodeAngle, Radius, RingIndexes};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCoordinate {
    pub cx: f32,
    pub cy: f32,
    pub radius: u32,
    pub x: f32,
    pub y: f32,
    pub node: String,
}

impl NodeCoordinate {
    /// Compute the coordinates of each node in the ring using the angle and radius.
    /// It will start on a given x, and y center
    /// x = cx + r * cos(<theta/radian>)
    /// y = cy + r * sin(<theta/radian>)
    pub fn get(
        nodes_angle: &Vec<NodeAngle>,
        ring_indexes: &RingIndexes,
        rings_radius: &Vec<Radius>,
        default_cx: Option<f32>,
        default_cy: Option<f32>,
    ) -> anyhow::Result<Vec<NodeCoordinate>> {
        let cx = default_cx.unwrap_or(0.0);
        let cy = default_cy.unwrap_or(0.0);
        let mut values: Vec<NodeCoordinate> = Vec::new();
        for n in nodes_angle {
            let ring_index = ring_indexes
                .values
                .iter()
                .find(|item| item.nodes.contains(&n.node))
                .unwrap()
                .index;
            let ring_radius = rings_radius
                .iter()
                .find(|item| item.ring == ring_index)
                .unwrap()
                .radius;
            let x = cx + ring_radius as f32 * n.angle_radian.cos();
            let y = cy + ring_radius as f32 * n.angle_radian.sin();
            values.push(NodeCoordinate {
                cx: cx.clone(),
                cy: cy.clone(),
                x,
                y,
                radius: ring_radius,
                node: n.node.clone(),
            });
        }
        Ok(values)
    }
}
