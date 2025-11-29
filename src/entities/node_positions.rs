use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePositionData {
    pub index: u32,
    pub radius: u32,
    pub angle_degree: f32,
    pub angle_radian: f32,
    pub cx: f32,
    pub cy: f32,
    pub x: f32,
    pub y: f32,
    pub node_id: String,
}
