use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NormalizeData {
    pub max_value: f32,
    pub values: Vec<NormalizeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizeValue {
    pub node_id: String,
    pub value: f32,
}
