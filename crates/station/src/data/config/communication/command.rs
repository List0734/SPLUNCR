use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationCommandConfig {
    pub target_address: String,
    pub send_rate_hz: u32,
    pub reconnect_interval_ms: u32,
}