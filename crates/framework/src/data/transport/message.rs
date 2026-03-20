use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T: Serialize> {
    pub timestamp: u64,
    pub payload: T,
}