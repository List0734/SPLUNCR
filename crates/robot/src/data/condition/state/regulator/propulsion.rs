pub mod thruster;
pub use thruster::{CoastRegulatorState, ThrusterRegulatorState};

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct PropulsionRegulatorState {
    pub thruster: ThrusterRegulatorState,
}

impl PropulsionRegulatorState {
    pub fn default() -> Self {
        Self {
            thruster: ThrusterRegulatorState::default(),
        }
    }
}