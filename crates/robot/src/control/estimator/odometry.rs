use nalgebra::Vector3;

use framework::control::integrators::exponential_map;
use framework::physics::constants::STANDARD_GRAVITY;
use framework::physics::kinematics::{Orientation, Pose, Twist};

use crate::platform::Fp;

pub struct Odometry {
	pose: Pose<Fp>,
	twist: Twist<Fp>,
}

impl Odometry {
	pub fn new() -> Self {
		Self {
			pose: Pose::identity(),
			twist: Twist::zero(),
		}
	}

	pub fn pose(&self) -> Pose<Fp> {
		self.pose
	}

	pub fn twist(&self) -> Twist<Fp> {
		self.twist
	}

	pub fn integrate(&mut self, dt: Fp) {
		exponential_map(&mut self.pose, &self.twist, dt);
	}

	pub fn apply_linear_acceleration(&mut self, acceleration: Vector3<Fp>, dt: Fp) {
		self.twist.linear += acceleration * dt;
	}

	pub fn apply_specific_force(&mut self, specific_force: Vector3<Fp>, dt: Fp) {
		let gravity_body = self.pose.rotation.inverse() * Vector3::z() * STANDARD_GRAVITY;
		self.apply_linear_acceleration(specific_force - gravity_body, dt);
	}

	pub fn update_angular_velocity(&mut self, velocity: Vector3<Fp>) {
		self.twist.angular = velocity;
	}

	pub fn update_orientation(&mut self, orientation: Orientation<Fp>) {
		self.pose.rotation = orientation;
	}
}
