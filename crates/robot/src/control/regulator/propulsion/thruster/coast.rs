use framework::control::controllers::slew_rate_limiter::{SlewRateLimiter, SlewRateLimiterConfig};

use crate::{
	data::config::{Config, propulsion::regulator::thruster::CoastRegulatorConfig},
	platform::{F, subsystem::propulsion::NUM_THRUSTERS},
};

pub struct CoastRegulator {
	limiters: [SlewRateLimiter<F>; NUM_THRUSTERS],
}

impl CoastRegulator {
	pub fn new(config: CoastRegulatorConfig) -> Self {
		let limiter_config = SlewRateLimiterConfig {
			rising_rate: config.accel_rate,
			falling_rate: config.decel_rate,
		};

		Self {
			limiters: [limiter_config; NUM_THRUSTERS].map(SlewRateLimiter::new),
		}
	}

	pub fn update(&mut self, commanded: &[F; NUM_THRUSTERS], dt: F) -> [F; NUM_THRUSTERS] {
		let mut output = [0.0; NUM_THRUSTERS];
		for (i, limiter) in self.limiters.iter_mut().enumerate() {
			output[i] = limiter.update(commanded[i], dt);
		}
		output
	}
}

impl Config<CoastRegulatorConfig> for CoastRegulator {
	fn update_config(&mut self, config: CoastRegulatorConfig) {
		let limiter_config = SlewRateLimiterConfig {
			rising_rate: config.accel_rate,
			falling_rate: config.decel_rate,
		};

		for limiter in &mut self.limiters {
			limiter.set_config(limiter_config);
		}
	}
}
