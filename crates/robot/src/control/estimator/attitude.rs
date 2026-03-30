use nalgebra::Vector3;

use framework::control::filters::Mahony;
use framework::physics::kinematics::Orientation;

use crate::data::config::sensor::estimator::AttitudeEstimatorConfig;
use crate::platform::Fp;

pub struct Attitude {
	filter: Mahony<Fp>,
	acceleration_tolerance: Fp,
}

impl Attitude {
	pub fn new(config: &AttitudeEstimatorConfig) -> Self {
		Self {
			filter: Mahony::new(config.mahony),
			acceleration_tolerance: config.acceleration_tolerance as Fp,
		}
	}

	pub fn initialize(&mut self, acceleration: Vector3<Fp>) {
		self.filter.initialize(acceleration);
	}

	pub fn update(&mut self, acceleration: Vector3<Fp>, angular_velocity: Vector3<Fp>, dt: Fp) {
		let gated = self.gate_acceleration(acceleration);
		self.filter.update(gated, angular_velocity, dt);
	}

	// Only use accelerometer for gravity correction when magnitude is near 1g
	fn gate_acceleration(&self, acceleration: Vector3<Fp>) -> Vector3<Fp> {
		if (acceleration.norm() - 1.0).abs() < self.acceleration_tolerance {
			acceleration
		} else {
			Vector3::zeros()
		}
	}

	pub fn orientation(&self) -> Orientation<Fp> {
		self.filter.orientation()
	}
}
