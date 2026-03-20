use nalgebra::Vector3;

use framework::physics::kinematics::{Pose, Twist};

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

	pub fn update(&mut self, dt: Fp) {
		self.integrate(dt);
	}

	pub fn pose(&self) -> Pose<Fp> {
		self.pose
	}

	pub fn twist(&self) -> Twist<Fp> {
		self.twist
	}

	fn integrate(&mut self, dt: Fp) {
		let delta = Pose::new(
			self.twist.linear * dt,
			self.twist.angular * dt,
		);

		self.pose *= delta;
	}

	pub fn apply_linear_acceleration(&mut self, acceleration: Vector3<Fp>, dt: Fp) {
		self.twist.linear += acceleration * dt;
	}

	pub fn update_angular_velocity(&mut self, velocity: Vector3<Fp>) {
		self.twist.angular = velocity;
	}
}
