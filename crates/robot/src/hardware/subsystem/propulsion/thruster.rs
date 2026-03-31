use framework::hardware::interface::Motor;
use framework::physics::kinematics::Placement;

use crate::data::config::propulsion::{MaxForce, ThrusterConfig};
use crate::data::config::Config;
use crate::platform::F;

pub struct Thruster<M: Motor<F>> {
	placement: Placement<F>,
	max_force: MaxForce,
	motor: M,
}

impl<M: Motor<F>> Thruster<M> {
	pub fn new(config: ThrusterConfig, motor: M) -> Self {
		let placement = Placement::from_arrays(config.placement.position, config.placement.direction);
		let max_force = config.max_force.expect("thruster max_force must be resolved before construction");

		Self {
			placement,
			max_force,
			motor,
		}
	}

	pub fn placement(&self) -> &Placement<F> {
		&self.placement
	}

	pub fn max_force(&self) -> MaxForce {
		self.max_force
	}

	pub fn init(&mut self) {
		self.motor.init().expect("failed to initialize motor");
		self.motor.set_enabled(true).expect("failed to enable motor");
	}

	pub fn set_force(&mut self, force: F) {
		let duty = if force >= 0.0 {
			(force / self.max_force.forward).clamp(0.0, 1.0).sqrt()
		} else {
			-((-force) / self.max_force.reverse).clamp(0.0, 1.0).sqrt()
		};
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
		self.max_force = config.max_force.expect("thruster max_force must be resolved before update");
	}
}
