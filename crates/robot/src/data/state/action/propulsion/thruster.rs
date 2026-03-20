pub mod coast;
pub use coast::CoastRegulatorState;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThrusterAction {
	pub coast: CoastRegulatorState,
}

impl Default for ThrusterAction {
	fn default() -> Self {
		Self {
			coast: CoastRegulatorState::default(),
		}
	}
}
