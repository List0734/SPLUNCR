use nalgebra::Vector3;

use framework::physics::kinematics::{Pose, Twist};
use robot::platform::Fp;

pub struct RigidBodyState {
	pub pose: Pose<Fp>,
	pub twist: Twist<Fp>,
	pub acceleration: Vector3<Fp>,
}

impl Default for RigidBodyState {
	fn default() -> Self {
		Self {
			pose: Pose::identity(),
			twist: Twist::zero(),
			acceleration: Vector3::zeros(),
		}
	}
}
