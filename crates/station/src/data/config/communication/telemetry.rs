use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationTelemetryConfig {
    pub listen_address: String,
}
