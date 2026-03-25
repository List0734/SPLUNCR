use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct AquaticState {
	pub depth: f32,
	pub pressure: f32,
	pub temperature: f32,
}
