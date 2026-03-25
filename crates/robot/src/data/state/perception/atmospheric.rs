use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct AtmosphericState {
	pub temperature: f32,
	pub pressure: f32,
}
