use serde::{Deserialize, Serialize};

use crate::platform::F;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct MaxForce {
	pub forward: F,
	pub reverse: F,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct ThrusterConfig {
	pub id: i8,
	pub gpio_pin: u8,
	pub placement: Placement,
	#[serde(default)]
	pub bidirectional: bool,
	pub max_force: Option<MaxForce>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Placement {
	pub position: [F; 3],
	pub direction: [F; 3],
}
