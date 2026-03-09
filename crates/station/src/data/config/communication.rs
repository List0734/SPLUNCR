mod command;
mod telemetry;
pub use command::StationCommandConfig;
pub use telemetry::StationTelemetryConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationCommunicationConfig {
    pub command: StationCommandConfig,
    pub telemetry: StationTelemetryConfig,
}