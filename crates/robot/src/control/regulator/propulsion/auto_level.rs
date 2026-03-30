use framework::control::controllers::PID;

use crate::data::config::{Config, propulsion::regulator::AutoLevelConfig};
use crate::platform::F;

pub struct AutoLevelRegulator {
	roll: PID<F>,
	pitch: PID<F>,
}

impl AutoLevelRegulator {
	pub fn new(config: AutoLevelConfig) -> Self {
		Self {
			roll: PID::from_config(config.roll),
			pitch: PID::from_config(config.pitch),
		}
	}

	pub fn update(&mut self, roll: F, pitch: F, dt: F) -> (F, F) {
		(
			self.roll.update(0.0, roll, dt),
			self.pitch.update(0.0, pitch, dt),
		)
	}
}

impl Config<AutoLevelConfig> for AutoLevelRegulator {
	fn update_config(&mut self, config: AutoLevelConfig) {
		self.roll.set_gains(config.roll.kp, config.roll.ki, config.roll.kd);
		self.pitch.set_gains(config.pitch.kp, config.pitch.ki, config.pitch.kd);
	}
}
