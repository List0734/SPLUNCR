use serde::{Deserialize, Serialize};

mod velocity;
pub use velocity::VelocityRegulatorConfig;

pub mod thruster;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PropulsionRegulatorConfig {
    pub velocity: VelocityRegulatorConfig,
    pub thruster: thruster::ThrusterRegulatorConfig,
}