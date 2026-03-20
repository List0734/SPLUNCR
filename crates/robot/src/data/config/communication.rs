mod command;
pub use command::CommandConfig;

mod telemetry;
pub use telemetry::TelemetryConfig;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommunicationConfig {
	pub poll_rate_hz: u32,
	pub command: CommandConfig,
	pub telemetry: TelemetryConfig,
}
