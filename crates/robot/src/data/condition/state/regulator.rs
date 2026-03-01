pub mod propulsion;
pub use propulsion::{CoastRegulatorState, PropulsionRegulatorState, VelocityRegulatorState};

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct RegulatorBundle {
    pub propulsion: PropulsionRegulatorState,
}

impl RegulatorBundle {
    pub fn default() -> Self {
        Self {
            propulsion: PropulsionRegulatorState::default(),
        }
    }
}
