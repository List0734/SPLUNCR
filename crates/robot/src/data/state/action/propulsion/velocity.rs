use serde::{Serialize, Deserialize};

use framework::physics::kinematics::Twist;

use crate::platform::F;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct VelocityRegulatorState {
	pub setpoint: Twist<F>,
	pub output: Twist<F>,
}

impl Default for VelocityRegulatorState {
	fn default() -> Self {
		Self {
			setpoint: Twist::zero(),
			output: Twist::zero(),
		}
	}
}
