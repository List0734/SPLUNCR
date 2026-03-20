use framework::hardware::interface::Motor;
use framework::physics::kinematics::Placement;

use crate::data::config::propulsion::ThrusterConfig;
use crate::data::config::Config;
use crate::platform::F;

pub struct Thruster<M: Motor<F>> {
	placement: Placement<F>,
	motor: M,
}

impl<M: Motor<F>> Thruster<M> {
	pub fn new(config: ThrusterConfig, motor: M) -> Self {
		let placement = Placement::from_arrays(config.placement.position, config.placement.direction);

		Self {
			placement,
			motor,
		}
	}

	pub fn placement(&self) -> &Placement<F> {
		&self.placement
	}

	pub fn init(&mut self) {
		self.motor.init().expect("failed to initialize motor");
		self.motor.set_enabled(true).expect("failed to enable motor");
	}

	pub fn set_duty_cycle(&mut self, duty: F) {
		let _ = self.motor.set_duty_cycle(duty);
	}

	pub fn stop(&mut self) {
		let _ = self.motor.set_duty_cycle(0.0);
		let _ = self.motor.set_enabled(false);
	}
}

impl<M: Motor<F>> Config<ThrusterConfig> for Thruster<M> {
	fn update_config(&mut self, config: ThrusterConfig) {
		self.placement = Placement::from_arrays(config.placement.position, config.placement.direction);
	}
}
