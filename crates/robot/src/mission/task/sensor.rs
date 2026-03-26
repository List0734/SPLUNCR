use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use nalgebra::Vector3;

use framework::hardware::interface::{Accelerometer, Gyroscope, Thermometer, Barometer, Bathometer};

use crate::control::estimator::{Attitude, Odometry};
use crate::data::state::perception::aquatic::AquaticState;
use crate::data::state::perception::atmospheric::AtmosphericState;
use crate::data::state::perception::navigation::odometry::OdometryEstimatorState;
use crate::hardware::subsystem::Imu;
use crate::mission::context::TaskContext;
use crate::platform::Fp;

pub struct SensorTask<I, A, W> {
	context: TaskContext,
	attitude: Attitude,
	odometry: Odometry,
	imu: Imu<I>,
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
		attitude: Attitude,
		odometry: Odometry,
		imu: Imu<I>,
		atmospheric_sensor: A,
		aquatic_sensor: W,
		rate_hz: u32,
	) -> Self {
		Self {
			context,
			attitude,
			odometry,
			imu,
			atmospheric_sensor,
			aquatic_sensor,
			period: Duration::from_secs_f64(1.0 / rate_hz as f64),
		}
	}

	fn initialize(&mut self) {
		if let Some(accel) = self.imu.read_acceleration() {
			self.attitude.initialize(accel.cast::<Fp>());
		}
	}

	pub fn run(mut self) {
		self.initialize();

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
		let acceleration = self.imu.read_acceleration().map(|a| a.cast::<Fp>());
		let rotation = self.imu.read_rotation().map(|r| r.cast::<Fp>());

		if let (Some(acceleration), Some(rotation)) = (acceleration, rotation) {
			self.attitude.update(acceleration, rotation, dt);
			self.odometry.update_orientation(self.attitude.orientation());

			self.odometry.apply_specific_force(acceleration, dt);
		}
		self.odometry.integrate(dt);

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
