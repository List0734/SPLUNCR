use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StationControllerConfig {
	pub poll_rate_hz: u32,
	pub deadband: f32,
}
