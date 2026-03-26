use nalgebra::{RealField, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct MahonyConfig<S> {
	pub kp: S,
	pub ki: S,
}

pub struct Mahony<S: RealField + Copy> {
	config: MahonyConfig<S>,
	orientation: UnitQuaternion<S>,
	integral_feedback: Vector3<S>,
}

impl<S: RealField + Copy> Mahony<S> {
	pub fn new(config: MahonyConfig<S>) -> Self {
		Self {
			config,
			orientation: UnitQuaternion::identity(),
			integral_feedback: Vector3::zeros(),
		}
	}

	pub fn update(&mut self, accel: Vector3<S>, gyro: Vector3<S>, dt: S) {
		let a = accel.normalize();
		let v = self.orientation.inverse() * Vector3::z();

		let error = a.cross(&v);

		self.integral_feedback += error * self.config.ki * dt;
		let corrected = gyro + error * self.config.kp + self.integral_feedback;

		self.orientation *= UnitQuaternion::new(corrected * dt);
	}

	pub fn initialize(&mut self, accel: Vector3<S>) {
		let a = accel.normalize();
		let roll = a.y.atan2(a.z);
		let pitch = (-a.x).atan2((a.y * a.y + a.z * a.z).sqrt());
		self.orientation = UnitQuaternion::from_euler_angles(roll, pitch, S::zero());
	}

	pub fn orientation(&self) -> UnitQuaternion<S> {
		self.orientation
	}

	pub fn set_config(&mut self, config: MahonyConfig<S>) {
		self.config = config;
	}

	pub fn reset(&mut self) {
		self.orientation = UnitQuaternion::identity();
		self.integral_feedback = Vector3::zeros();
	}
}