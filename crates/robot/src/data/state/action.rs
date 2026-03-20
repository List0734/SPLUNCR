pub mod propulsion;

use serde::{Serialize, Deserialize};

pub use propulsion::PropulsionAction;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionState {
	pub propulsion: PropulsionAction,
}

impl Default for ActionState {
	fn default() -> Self {
		Self {
			propulsion: PropulsionAction::default(),
		}
	}
}
