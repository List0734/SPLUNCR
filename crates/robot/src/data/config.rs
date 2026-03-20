pub mod propulsion;
pub mod communication;
pub mod vision;
pub mod sensor;

use serde::{Deserialize, Serialize};

pub use propulsion::PropulsionConfig;
pub use communication::CommunicationConfig;
pub use vision::VisionConfig;
pub use sensor::SensorConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigBundle {
	pub propulsion: PropulsionConfig,
	pub communication: CommunicationConfig,
	pub vision: VisionConfig,
	pub sensor: SensorConfig,
}

pub trait Config<C> {
	fn update_config(&mut self, config: C);
}
