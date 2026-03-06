mod communication;
pub use communication::{StationCommunicationConfig, StationTelemetryConfig};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationConfig {
    pub communication: StationCommunicationConfig,
}
