use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoConfig {
    pub target_address: String,
    pub bind_address: String,
}
