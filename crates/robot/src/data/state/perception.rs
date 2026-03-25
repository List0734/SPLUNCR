pub mod aquatic;
pub mod atmospheric;
pub mod navigation;

use serde::{Serialize, Deserialize};

pub use aquatic::AquaticState;
pub use atmospheric::AtmosphericState;
pub use navigation::NavigationPerception;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerceptionState {
	pub navigation: NavigationPerception,
	pub atmospheric: AtmosphericState,
	pub aquatic: AquaticState,
}

impl Default for PerceptionState {
	fn default() -> Self {
		Self {
			navigation: NavigationPerception::default(),
			atmospheric: AtmosphericState::default(),
			aquatic: AquaticState::default(),
		}
	}
}
