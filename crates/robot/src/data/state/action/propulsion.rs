pub mod velocity;
pub mod thruster;

use serde::{Serialize, Deserialize};

pub use velocity::VelocityRegulatorState;
pub use thruster::ThrusterAction;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropulsionAction {
	pub velocity: VelocityRegulatorState,
	pub thruster: ThrusterAction,
}

impl Default for PropulsionAction {
	fn default() -> Self {
		Self {
			velocity: VelocityRegulatorState::default(),
			thruster: ThrusterAction::default(),
		}
	}
}
