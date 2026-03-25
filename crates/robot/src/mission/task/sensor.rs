use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use nalgebra::Vector3;

use framework::hardware::interface::{Accelerometer, Gyroscope, Thermometer, Barometer, Bathometer};

use crate::control::estimator::Odometry;
use crate::data::state::perception::aquatic::AquaticState;
use crate::data::state::perception::atmospheric::AtmosphericState;
use crate::data::state::perception::navigation::odometry::OdometryEstimatorState;
use crate::mission::context::TaskContext;
use crate::platform::Fp;

pub struct SensorTask<I, A, W> {
	context: TaskContext,
	odometry: Odometry,
	imu: I,
	atmospheric_sensor: A,
	aquatic_sensor: W,
	period: Duration,
}

impl<I, A, W> SensorTask<I, A, W>
where
	I: Accelerometer<Vector3<f32>> + Gyroscope<Vector3<f32>>,
	A: Thermometer<f32> + Barometer<f32>,
	W: Bathometer<f32> + Thermometer<f32> + Barometer<f32>,
{
	pub fn new(
		context: TaskContext,
		odometry: Odometry,
		imu: I,
		atmospheric_sensor: A,
		aquatic_sensor: W,
		rate_hz: u32,
	) -> Self {
		Self {
			context,
			odometry,
			imu,
			atmospheric_sensor,
			aquatic_sensor,
			period: Duration::from_secs_f64(1.0 / rate_hz as f64),
		}
	}

	pub fn run(mut self) {
		let mut last = Instant::now();

		while !self.context.shutdown.load(Ordering::Relaxed) {
			let now = Instant::now();
			let dt = now.duration_since(last).as_secs_f64();
			last = now;

			self.step(dt);

			let elapsed = now.elapsed();
			if elapsed < self.period {
				thread::sleep(self.period - elapsed);
			}
		}
	}

	fn step(&mut self, dt: Fp) {
		if let Ok(accel) = self.imu.read_acceleration() {
			self.odometry.apply_linear_acceleration(accel.cast::<Fp>(), dt);
		}
		if let Ok(gyro) = self.imu.read_rotation() {
			self.odometry.update_angular_velocity(gyro.cast::<Fp>());
		}
		self.odometry.update(dt);

		let atmospheric = AtmosphericState {
			temperature: self.atmospheric_sensor.read_temperature().unwrap_or(0.0),
			pressure: self.atmospheric_sensor.read_pressure().unwrap_or(0.0),
		};

		let mut state = self.context.state.write().unwrap();
		state.perception.navigation.odometry = OdometryEstimatorState {
			pose: self.odometry.pose(),
			twist: self.odometry.twist(),
		};
		state.perception.atmospheric = atmospheric;
		state.perception.aquatic = AquaticState {
			depth: self.aquatic_sensor.read_depth().unwrap_or(0.0),
			pressure: self.aquatic_sensor.read_pressure().unwrap_or(0.0),
			temperature: self.aquatic_sensor.read_temperature().unwrap_or(0.0),
		};
	}
}
