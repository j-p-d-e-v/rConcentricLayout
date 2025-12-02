use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct Edge {
    pub id: u32,
    pub source_id: u32,
    pub target_id: u32,
}
