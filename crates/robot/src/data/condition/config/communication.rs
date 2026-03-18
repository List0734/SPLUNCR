mod command;
pub use command::CommandConfig;

mod telemetry;
pub use telemetry::TelemetryConfig;

mod video;
pub use video::VideoConfig;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommunicationConfig {
    pub command: CommandConfig,
    pub telemetry: TelemetryConfig,
    pub video: VideoConfig,
}
