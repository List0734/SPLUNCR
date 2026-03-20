use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use crate::control::estimator::Odometry;
use crate::data::state::perception::navigation::odometry::OdometryEstimatorState;
use crate::mission::context::TaskContext;
use crate::platform::Fp;

pub struct SensorTask {
	context: TaskContext,
	odometry: Odometry,
	period: Duration,
}

impl SensorTask {
	pub fn new(
		context: TaskContext,
		odometry: Odometry,
		rate_hz: u32,
	) -> Self {
		Self {
			context,
			odometry,
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
		self.odometry.update(dt);

		{
			let mut state = self.context.state.write().unwrap();
			state.perception.navigation.odometry = OdometryEstimatorState {
				pose: self.odometry.pose(),
				twist: self.odometry.twist(),
			};
		}
	}
}
