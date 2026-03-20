mod command;
mod telemetry;
mod video;
pub use command::StationCommandConfig;
pub use telemetry::StationTelemetryConfig;
pub use video::StationVideoConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationCommunicationConfig {
    pub command: StationCommandConfig,
    pub telemetry: StationTelemetryConfig,
    pub video: StationVideoConfig,
    pub poll_rate_hz: u32,
}