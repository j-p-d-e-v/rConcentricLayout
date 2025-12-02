use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NormalizeData {
    pub max_value: f32,
    pub values: Vec<NormalizeValue>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct NormalizeValue {
    pub node_id: u32,
    pub value: f32,
}
