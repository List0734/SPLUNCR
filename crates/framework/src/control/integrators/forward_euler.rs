use nalgebra::Vector3;

use crate::physics::kinematics::{Orientation, Pose, Twist};

pub fn forward_euler<S: nalgebra::RealField + serde::Serialize + Copy>(
	pose: &mut Pose<S>,
	twist: &Twist<S>,
	dt: S,
) {
	let delta = Pose::new(twist.linear * dt, Vector3::zeros());
	*pose *= delta;
	pose.rotation *= Orientation::new(twist.angular * dt);
}
