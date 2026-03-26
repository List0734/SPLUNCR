use framework::hardware::interface::Motor;
use framework::physics::dynamics::Wrench;

use crate::data::config::propulsion::PropulsionConfig;
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
		let thrusters = config.thrusters.map(|config| {
			Thruster::new(config, motors_iter.next().unwrap())
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

	pub fn set_duty_cycles(&mut self, duties: &[F; NUM_THRUSTERS]) {
		for (thruster, &duty) in self.thrusters.iter_mut().zip(duties.iter()) {
			thruster.set_duty_cycle(duty);
		}
	}

	pub fn stop(&mut self) {
		for thruster in &mut self.thrusters {
			thruster.stop();
		}
	}
}

impl<M: Motor<F>> Config<PropulsionConfig> for PropulsionSubsystem<M> {
	fn update_config(&mut self, config: PropulsionConfig) {
		for (thruster, config) in self.thrusters.iter_mut().zip(&config.thrusters) {
			thruster.update_config(*config);
		}
		self.allocator = Allocator::new(&self.thrusters);
	}
}
