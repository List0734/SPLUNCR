use serde::{Deserialize, Serialize};

use framework::control::controllers::pid::PIDConfig;

mod velocity;
pub use velocity::VelocityRegulatorConfig;

pub mod thruster;

use crate::platform::F;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PropulsionRegulatorConfig {
	pub velocity: VelocityRegulatorConfig,
	pub thruster: thruster::ThrusterRegulatorConfig,
	pub depth_hold: DepthHoldConfig,
	pub auto_level: AutoLevelConfig,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct DepthHoldConfig {
	pub depth_rate: PIDConfig<F>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AutoLevelConfig {
	pub roll: PIDConfig<F>,
	pub pitch: PIDConfig<F>,
}
