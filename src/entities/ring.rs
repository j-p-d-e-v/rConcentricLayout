use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RingData {
    pub index: u32, //Sequential Index
    pub original_index: u32,
    pub nodes: Vec<String>,
    pub radius: u32,
}
