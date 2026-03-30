mod auto_level;
mod depth_hold;
mod velocity;

pub use auto_level::AutoLevelRegulator;
pub use depth_hold::DepthHoldRegulator;
pub use velocity::VelocityRegulator;

pub mod thruster;

use crate::data::config::propulsion::regulator::PropulsionRegulatorConfig;

pub struct PropulsionRegulator {
	pub velocity: VelocityRegulator,
	pub thruster: thruster::CoastRegulator,
	pub depth_hold: DepthHoldRegulator,
	pub auto_level: AutoLevelRegulator,
}

impl PropulsionRegulator {
	pub fn new(config: PropulsionRegulatorConfig) -> Self {
		Self {
			velocity: VelocityRegulator::new(config.velocity),
			thruster: thruster::CoastRegulator::new(config.thruster.coast),
			depth_hold: DepthHoldRegulator::new(config.depth_hold),
			auto_level: AutoLevelRegulator::new(config.auto_level),
		}
	}
}
