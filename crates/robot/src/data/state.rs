pub mod perception;
pub mod autonomous;
pub mod action;

use serde::{Serialize, Deserialize};

pub use perception::PerceptionState;
pub use autonomous::AutonomousCommand;
pub use action::ActionState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RobotState {
	pub perception: PerceptionState,
	pub autonomous: AutonomousCommand,
	pub action: ActionState,
}

impl Default for RobotState {
	fn default() -> Self {
		Self {
			perception: PerceptionState::default(),
			autonomous: AutonomousCommand::default(),
			action: ActionState::default(),
		}
	}
}
