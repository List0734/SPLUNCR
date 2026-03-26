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

	pub fn allocate(&self, wrench: Wrench<F>, bidirectional_thrust: bool) -> [F; NUM_THRUSTERS] {
		let mut reverse_allowed = [false; NUM_THRUSTERS];
		for (i, thruster) in self.thrusters.iter().enumerate() {
			reverse_allowed[i] = bidirectional_thrust || thruster.bidirectional();
		}
		self.allocator.allocate(wrench, reverse_allowed)
	}

	pub fn set_thrust_fractions(&mut self, fractions: &[F; NUM_THRUSTERS]) {
		for (thruster, &fraction) in self.thrusters.iter_mut().zip(fractions.iter()) {
			thruster.set_thrust_fraction(fraction);
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
