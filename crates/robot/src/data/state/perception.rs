pub mod navigation;

use serde::{Serialize, Deserialize};

pub use navigation::NavigationPerception;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerceptionState {
	pub navigation: NavigationPerception,
}

impl Default for PerceptionState {
	fn default() -> Self {
		Self {
			navigation: NavigationPerception::default(),
		}
	}
}
