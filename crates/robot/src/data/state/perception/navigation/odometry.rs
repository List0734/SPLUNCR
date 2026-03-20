use serde::{Serialize, Deserialize};

use framework::physics::kinematics::{Pose, Twist};

use crate::platform::Fp;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct OdometryEstimatorState {
	pub pose: Pose<Fp>,
	pub twist: Twist<Fp>,
}

impl Default for OdometryEstimatorState {
	fn default() -> Self {
		Self {
			pose: Pose::identity(),
			twist: Twist::zero(),
		}
	}
}
