use serde::{Deserialize, Serialize};

use crate::platform::F;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct ThrusterConfig {
	pub id: i8,
	pub gpio_pin: u8,
	pub placement: Placement,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Placement {
	pub position: [F; 3],
	pub direction: [F; 3],
}
