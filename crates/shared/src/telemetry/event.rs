use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub timestamp_ms: u64,
    pub path: String,
    pub value: Value,
}

impl Event {
    pub fn new(path: String, value: Value) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            timestamp_ms,
            path,
            value,
        }
    }
}