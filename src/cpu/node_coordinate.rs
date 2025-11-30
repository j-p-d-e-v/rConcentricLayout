use crate::{cpu::NodeAngle, entities::RingData};
use anyhow::anyhow;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCoordinate {
    pub cx: f32,
    pub cy: f32,
    pub radius: u32,
    pub x: f32,
    pub y: f32,
    pub node_id: String,
}

impl NodeCoordinate {
    /// Compute the coordinates of each node in the ring using the angle and radius.
    /// It will start on a given x, and y center
    /// x = cx + r * cos(<theta/radian>)
    /// y = cy + r * sin(<theta/radian>)
    pub fn get(
        nodes_angle: &Vec<NodeAngle>,
        rings: &[RingData],
        default_cx: Option<f32>,
        default_cy: Option<f32>,
    ) -> anyhow::Result<Vec<NodeCoordinate>> {
        let cx = default_cx.unwrap_or(0.0);
        let cy = default_cy.unwrap_or(0.0);
        let values = std::panic::catch_unwind(|| {
            Ok(nodes_angle
                .par_iter()
                .map(|n| {
                    let ring = rings
                        .iter()
                        .find(|item| item.nodes.contains(&n.node))
                        .unwrap();
                    let ring_radius = ring.radius;

                    let x = cx + ring_radius as f32 * n.angle_radian.cos();
                    let y = cy + ring_radius as f32 * n.angle_radian.sin();
                    NodeCoordinate {
                        cx,
                        cy,
                        x,
                        y,
                        radius: ring_radius,
                        node_id: n.node.clone(),
                    }
                })
                .collect::<Vec<NodeCoordinate>>())
        });
        match values {
            Ok(data) => data,
            Err(_) => Err(anyhow!("unable to calculate nodes coorindate")),
        }
    }
}
