mod velocity;
pub use velocity::VelocityRegulator;

pub mod thruster;

use crate::{data::condition::config::regulator::PropulsionConfig, data::transport::telemetry::Publisher};

pub struct PropulsionRegulator {
    pub velocity: VelocityRegulator,
    pub thruster: thruster::CoastRegulator,
}

impl PropulsionRegulator {
    pub fn new(config: PropulsionConfig, telemetry: Publisher) -> Self {
        Self {
            velocity: VelocityRegulator::new(config.velocity),
            thruster: thruster::CoastRegulator::new(config.thruster, telemetry),
        }
    }
}