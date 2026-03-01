mod coast;
pub use coast::CoastRegulatorState;

use serde::Serialize;

use crate::platform::subsystem::propulsion::NUM_THRUSTERS;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ThrusterRegulatorState {
    pub coast: CoastRegulatorState,
}

impl ThrusterRegulatorState {
    pub fn default() -> Self {
        Self {
            coast: CoastRegulatorState {
                commanded: [0.0; NUM_THRUSTERS],
                output: [0.0; NUM_THRUSTERS],
            },
        }
    }
}
