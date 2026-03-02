mod coast;
pub use coast::CoastRegulatorState;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ThrusterRegulatorState {
    pub coast: CoastRegulatorState,
}

impl ThrusterRegulatorState {
    pub fn default() -> Self {
        Self {
            coast: CoastRegulatorState::default(),
        }
    }
}
