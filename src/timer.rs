use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Timer {
    pub micros: Option<u128>,
    pub millis: Option<u128>,
    pub seconds: Option<u64>,
}
