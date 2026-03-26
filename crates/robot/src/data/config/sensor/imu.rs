use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImuConfig {
	pub calibration_samples: usize,
	pub calibration_delay_ms: u32,
}
