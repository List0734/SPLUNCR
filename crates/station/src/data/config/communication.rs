mod telemetry;
pub use telemetry::StationTelemetryConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationCommunicationConfig {
    pub telemetry: StationTelemetryConfig,
}
