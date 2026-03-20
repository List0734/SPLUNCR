mod communication;
mod controller;
pub use communication::{StationCommunicationConfig, StationCommandConfig, StationTelemetryConfig, StationVideoConfig};
pub use controller::StationControllerConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationConfig {
    pub communication: StationCommunicationConfig,
    pub controller: StationControllerConfig,
}
