use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorConfig {
	pub loop_rate_hz: u32,
}
