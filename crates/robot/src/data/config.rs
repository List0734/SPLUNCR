pub mod body;
pub mod propulsion;
pub mod communication;
pub mod vision;
pub mod sensor;

use serde::{Deserialize, Serialize};

pub use body::BodyConfig;
pub use propulsion::PropulsionConfig;
pub use communication::CommunicationConfig;
pub use vision::VisionConfig;
pub use sensor::SensorConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RobotConfig {
	pub body: BodyConfig,
	pub propulsion: PropulsionConfig,
	pub communication: CommunicationConfig,
	pub vision: VisionConfig,
	pub sensor: SensorConfig,
}

pub trait Config<C> {
	fn update_config(&mut self, config: C);
}
