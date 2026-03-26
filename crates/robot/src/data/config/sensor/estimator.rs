use serde::{Deserialize, Serialize};

mod attitude;
pub use attitude::AttitudeEstimatorConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorEstimatorConfig {
	pub attitude: AttitudeEstimatorConfig,
}
