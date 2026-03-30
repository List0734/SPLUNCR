use framework::control::controllers::PID;

use crate::data::config::{Config, propulsion::regulator::DepthHoldConfig};
use crate::platform::F;

pub struct DepthHoldRegulator {
	depth_rate: PID<F>,
}

impl DepthHoldRegulator {
	pub fn new(config: DepthHoldConfig) -> Self {
		Self {
			depth_rate: PID::from_config(config.depth_rate),
		}
	}

	pub fn update(&mut self, setpoint: F, measured: F, dt: F) -> F {
		self.depth_rate.update(setpoint, measured, dt)
	}
}

impl Config<DepthHoldConfig> for DepthHoldRegulator {
	fn update_config(&mut self, config: DepthHoldConfig) {
		self.depth_rate.set_gains(config.depth_rate.kp, config.depth_rate.ki, config.depth_rate.kd);
	}
}
