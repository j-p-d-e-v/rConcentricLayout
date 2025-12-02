use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct Node {
    pub id: u32,
}
