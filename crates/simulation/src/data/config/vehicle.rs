mod hardware;
pub use hardware::HardwareConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VehicleConfig {
	pub hardware: HardwareConfig,
}
