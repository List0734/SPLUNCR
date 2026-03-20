use serde::{Serialize, Deserialize};

use crate::platform::{F, subsystem::propulsion::NUM_THRUSTERS};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoastRegulatorState {
	pub commanded: [F; NUM_THRUSTERS],
	pub output: [F; NUM_THRUSTERS],
}

impl Default for CoastRegulatorState {
	fn default() -> Self {
		Self {
			commanded: [0.0; NUM_THRUSTERS],
			output: [0.0; NUM_THRUSTERS],
		}
	}
}
