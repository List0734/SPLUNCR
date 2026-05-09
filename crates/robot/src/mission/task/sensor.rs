use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use nalgebra::Vector3;

use framework::hardware::interface::{Accelerometer, Gyroscope, Thermometer, Barometer, Bathometer};
use framework::physics::kinematics::{Pose, Twist};

use crate::control::estimator::{Attitude, HeaveEstimator};
use crate::data::state::perception::aquatic::AquaticState;
use crate::data::state::perception::atmospheric::AtmosphericState;
use crate::data::state::perception::navigation::odometry::OdometryState;
use crate::subsystem::Imu;
use crate::mission::context::TaskContext;
use crate::platform::Fp;

pub struct SensorTask<I, A, W> {
	context: TaskContext,
	attitude: Attitude,
	heave: HeaveEstimator,
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
		heave: HeaveEstimator,
		imu: Imu<I>,
		atmospheric_sensor: A,
		aquatic_sensor: W,
		rate_hz: u32,
	) -> Self {
		Self {
			context,
			attitude,
			heave,
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
		}

		let depth = self.aquatic_sensor.read_depth().unwrap_or(0.0) as Fp;
		self.heave.update(depth, dt);

		let mut pose = Pose::<Fp>::identity();
		pose.rotation = self.attitude.orientation();
		pose.translation.z = -depth;

		let twist = Twist {
			linear: Vector3::new(0.0, 0.0, -self.heave.velocity()),
			angular: rotation.unwrap_or_default(),
		};

		let atmospheric = AtmosphericState {
			temperature: self.atmospheric_sensor.read_temperature().unwrap_or(0.0),
			pressure: self.atmospheric_sensor.read_pressure().unwrap_or(0.0),
		};

		let mut state = self.context.state.write().unwrap();
		state.perception.navigation.odometry = OdometryState { pose, twist };
		state.perception.atmospheric = atmospheric;
		state.perception.aquatic = AquaticState {
			pressure: self.aquatic_sensor.read_pressure().unwrap_or(0.0),
			temperature: self.aquatic_sensor.read_temperature().unwrap_or(0.0),
		};
	}
}
