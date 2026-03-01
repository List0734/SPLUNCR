use serde::{Serialize, Deserialize};

use crate::platform::{F, subsystem::propulsion::NUM_THRUSTERS};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoastRegulatorState {
    pub commanded: [F; NUM_THRUSTERS],
    pub output: [F; NUM_THRUSTERS],
}
