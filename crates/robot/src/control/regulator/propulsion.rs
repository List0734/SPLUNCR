mod velocity;
pub use velocity::VelocityRegulator;

pub mod thruster;

use crate::{data::condition::config::regulator::PropulsionRegulatorConfig, data::transport::telemetry::Publisher};

pub struct PropulsionRegulator {
    pub velocity: VelocityRegulator,
    pub thruster: thruster::CoastRegulator,
}

impl PropulsionRegulator {
    pub fn new(config: PropulsionRegulatorConfig, telemetry: Publisher) -> Self {
        Self {
            velocity: VelocityRegulator::new(config.velocity, telemetry.clone()),
            thruster: thruster::CoastRegulator::new(config.thruster.coast, telemetry),
        }
    }
}