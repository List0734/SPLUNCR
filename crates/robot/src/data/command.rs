use serde::{Deserialize, Serialize};

use framework::physics::dynamics::Wrench;
use framework::physics::kinematics::Twist;

use crate::platform::F;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorCommand {
	pub propulsion: PropulsionCommand,
	pub bidirectional_thrust: bool,
	pub auto_level: bool,
}

impl Default for OperatorCommand {
	fn default() -> Self {
		Self {
			propulsion: PropulsionCommand::OpenLoop(Wrench::zero()),
			bidirectional_thrust: false,
			auto_level: false,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropulsionCommand {
	Velocity(Twist<F>),
	OpenLoop(Wrench<F>),
	DepthHold(DepthHoldCommand),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepthHoldCommand {
	pub wrench: Wrench<F>,
	pub depth_rate: F,
}
