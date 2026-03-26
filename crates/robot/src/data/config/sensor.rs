use serde::{Deserialize, Serialize};

pub mod estimator;
pub mod imu;

pub use estimator::SensorEstimatorConfig;
pub use imu::ImuConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorConfig {
	pub loop_rate_hz: u32,
	pub imu: ImuConfig,
	pub estimator: SensorEstimatorConfig,
}
