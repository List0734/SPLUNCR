use nalgebra::Vector3;

use framework::hardware::interface::{Accelerometer, Gyroscope};
use framework::physics::constants::STANDARD_GRAVITY;

use crate::data::config::Config;
use crate::data::config::sensor::ImuConfig;
use crate::platform::F;

pub struct Imu<I> {
	driver: I,
	accel_offset: Vector3<F>,
	gyro_offset: Vector3<F>,
}

impl<I> Imu<I>
where
	I: Accelerometer<Vector3<F>> + Gyroscope<Vector3<F>>,
{
	pub fn new(mut driver: I, config: &ImuConfig, delay: impl Fn(u32)) -> Self {
		let mut accel_sum = Vector3::<F>::zeros();
		let mut gyro_sum = Vector3::<F>::zeros();

		for _ in 0..config.calibration_samples {
			if let Ok(accel) = driver.read_acceleration() {
				accel_sum += accel;
			}
			if let Ok(gyro) = driver.read_rotation() {
				gyro_sum += gyro;
			}
			delay(config.calibration_delay_ms);
		}

		let n = config.calibration_samples as F;
		let accel_mean = accel_sum / n;
		let gyro_mean = gyro_sum / n;

		Self {
			driver,
			accel_offset: accel_mean - accel_mean.normalize(),
			gyro_offset: gyro_mean,
		}
	}

	pub fn read_acceleration(&mut self) -> Option<Vector3<F>> {
		self.driver.read_acceleration().ok().map(|a| (a - self.accel_offset) * STANDARD_GRAVITY as F)
	}

	pub fn read_rotation(&mut self) -> Option<Vector3<F>> {
		self.driver.read_rotation().ok().map(|r| r - self.gyro_offset)
	}
}

impl<I> Config<ImuConfig> for Imu<I> {
	fn update_config(&mut self, _config: ImuConfig) {}
}
