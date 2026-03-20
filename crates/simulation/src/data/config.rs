mod environment;
pub use environment::EnvironmentConfig;

mod vehicle;
pub use vehicle::VehicleConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimConfig {
	pub environment: EnvironmentConfig,
	pub vehicle: VehicleConfig,
}
