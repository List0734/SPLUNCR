use serde::{Deserialize, Serialize};
use robot::platform::F;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HardwareConfig {
	pub imu: SensorModelConfig,
	pub depth: SensorModelConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorModelConfig {
	pub noise_stddev: F,
}
