use nalgebra::{Matrix3, RealField, Vector3};

use crate::physics::kinematics::{Orientation, Pose, Twist};

pub fn exponential_map<S: RealField + serde::Serialize + Copy>(
	pose: &mut Pose<S>,
	twist: &Twist<S>,
	dt: S,
) {
	let omega_dt = twist.angular * dt;
	let angle = omega_dt.norm();
	let j = left_jacobian(&omega_dt, angle);
	let delta_position = j * (twist.linear * dt);
	let delta_rotation = Orientation::new(omega_dt);
	*pose *= Pose::new(delta_position, Vector3::zeros());
	pose.rotation *= delta_rotation;
}

fn left_jacobian<S: RealField + Copy>(omega_dt: &Vector3<S>, angle: S) -> Matrix3<S> {
	let eps = S::default_epsilon() * nalgebra::convert(1e6);
	if angle < eps {
		return Matrix3::identity();
	}

	let skew = omega_dt.cross_matrix();
	let angle_sq = angle * angle;

	let rotation_term = skew * ((S::one() - angle.cos()) / angle_sq);
	let coupling_term = (skew * skew) * ((angle - angle.sin()) / (angle_sq * angle));

	Matrix3::identity() + rotation_term + coupling_term
}
