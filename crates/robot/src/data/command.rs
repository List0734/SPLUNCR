use serde::{Deserialize, Serialize};

use framework::physics::dynamics::Wrench;
use framework::physics::kinematics::Twist;

use crate::platform::F;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorCommand {
	pub propulsion: PropulsionCommand,
}

impl Default for OperatorCommand {
	fn default() -> Self {
		Self {
			propulsion: PropulsionCommand::OpenLoop(Wrench::zero()),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropulsionCommand {
	Velocity(Twist<F>),
	OpenLoop(Wrench<F>),
}
