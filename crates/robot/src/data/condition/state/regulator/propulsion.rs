pub mod thruster;
pub use thruster::{CoastRegulatorState, ThrusterRegulatorState};

mod velocity;
pub use velocity::VelocityRegulatorState;

use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct PropulsionRegulatorState {
    pub velocity: VelocityRegulatorState,
    pub thruster: ThrusterRegulatorState,
}

impl PropulsionRegulatorState {
    pub fn default() -> Self {
        Self {
            velocity: VelocityRegulatorState::default(),
            thruster: ThrusterRegulatorState::default(),
        }
    }
}