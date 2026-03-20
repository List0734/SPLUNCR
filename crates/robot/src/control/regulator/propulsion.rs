mod velocity;
pub use velocity::VelocityRegulator;

pub mod thruster;

use crate::data::config::propulsion::regulator::PropulsionRegulatorConfig;

pub struct PropulsionRegulator {
	pub velocity: VelocityRegulator,
	pub thruster: thruster::CoastRegulator,
}

impl PropulsionRegulator {
	pub fn new(config: PropulsionRegulatorConfig) -> Self {
		Self {
			velocity: VelocityRegulator::new(config.velocity),
			thruster: thruster::CoastRegulator::new(config.thruster.coast),
		}
	}
}
