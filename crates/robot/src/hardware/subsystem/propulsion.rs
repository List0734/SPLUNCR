use framework::hardware::interface::Motor;
use framework::physics::dynamics::Wrench;

use crate::data::config::propulsion::{PropulsionConfig, ThrusterConfig};
use crate::data::config::Config;
use crate::platform::{F, subsystem::propulsion::NUM_THRUSTERS};

mod thruster;
pub use thruster::Thruster;
mod allocator;
pub use allocator::Allocator;

pub struct PropulsionSubsystem<M: Motor<F>> {
	thrusters: [Thruster<M>; NUM_THRUSTERS],
	allocator: Allocator,
}

impl<M: Motor<F>> PropulsionSubsystem<M> {
	pub fn new(config: PropulsionConfig, motors: [M; NUM_THRUSTERS]) -> Self {
		let mut motors_iter = motors.into_iter();
		let thrusters = Self::resolve_thruster_configs(&config).map(|thruster_config| {
			Thruster::new(thruster_config, motors_iter.next().unwrap())
		});
		let allocator = Allocator::new(&thrusters);

		Self {
			thrusters,
			allocator,
		}
	}

	pub fn init(&mut self) {
		for thruster in &mut self.thrusters {
			thruster.init();
		}
	}

	pub fn max_wrench(&self) -> &Wrench<F> {
		self.allocator.max_wrench()
	}

	pub fn allocate(&self, wrench: Wrench<F>) -> [F; NUM_THRUSTERS] {
		self.allocator.allocate(wrench)
	}

	pub fn set_forces(&mut self, forces: &[F; NUM_THRUSTERS]) {
		for (thruster, &force) in self.thrusters.iter_mut().zip(forces.iter()) {
			thruster.set_force(force);
		}
	}

	pub fn stop(&mut self) {
		for thruster in &mut self.thrusters {
			thruster.stop();
		}
	}

	fn resolve_thruster_configs(config: &PropulsionConfig) -> [ThrusterConfig; NUM_THRUSTERS] {
		let mut thrusters = config.thrusters;
		for thruster in &mut thrusters {
			if thruster.max_force.is_none() {
				thruster.max_force = Some(config.default_max_force);
			}
		}
		thrusters
	}
}

impl<M: Motor<F>> Config<PropulsionConfig> for PropulsionSubsystem<M> {
	fn update_config(&mut self, config: PropulsionConfig) {
		let resolved = PropulsionSubsystem::<M>::resolve_thruster_configs(&config);
		for (thruster, thruster_config) in self.thrusters.iter_mut().zip(&resolved) {
			thruster.update_config(*thruster_config);
		}
		self.allocator = Allocator::new(&self.thrusters);
	}
}
