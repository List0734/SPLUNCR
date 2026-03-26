use serde::{Deserialize, Serialize};

mod thruster;
pub use thruster::{MaxForce, ThrusterConfig};

pub mod regulator;
pub use regulator::PropulsionRegulatorConfig;

use crate::platform::subsystem::propulsion::NUM_THRUSTERS;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PropulsionConfig {
	pub loop_rate_hz: u32,
	pub default_max_force: MaxForce,
	pub thrusters: [ThrusterConfig; NUM_THRUSTERS],
	pub regulator: PropulsionRegulatorConfig,
}
