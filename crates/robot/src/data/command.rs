use serde::{Deserialize, Serialize};

use framework::physics::dynamics::Wrench;
use framework::physics::kinematics::Twist;

use crate::platform::F;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorCommand {
	pub propulsion: PropulsionCommand,
	pub auto_level: bool,
}

impl Default for OperatorCommand {
	fn default() -> Self {
		Self {
			propulsion: PropulsionCommand::OpenLoop(Wrench::zero()),
			auto_level: false,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropulsionCommand {
	Velocity(Twist<F>),
	OpenLoop(Wrench<F>),
	Stabilized(StabilizedCommand),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilizedCommand {
	pub wrench: Wrench<F>,
	pub depth_rate: F,
}
