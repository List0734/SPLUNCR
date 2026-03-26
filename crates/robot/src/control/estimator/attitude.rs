use nalgebra::Vector3;

use framework::control::filters::Mahony;
use framework::physics::kinematics::Orientation;

use crate::data::config::sensor::estimator::AttitudeEstimatorConfig;
use crate::platform::Fp;

pub struct Attitude {
	filter: Mahony<Fp>,
}

impl Attitude {
	pub fn new(config: &AttitudeEstimatorConfig) -> Self {
		Self {
			filter: Mahony::new(*config),
		}
	}

	pub fn initialize(&mut self, acceleration: Vector3<Fp>) {
		self.filter.initialize(acceleration);
	}

	pub fn update(&mut self, acceleration: Vector3<Fp>, angular_velocity: Vector3<Fp>, dt: Fp) {
		self.filter.update(acceleration, angular_velocity, dt);
	}

	pub fn orientation(&self) -> Orientation<Fp> {
		self.filter.orientation()
	}
}
