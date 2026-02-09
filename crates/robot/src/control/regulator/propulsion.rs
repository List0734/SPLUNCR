mod velocity;
pub use velocity::VelocityRegulator;

use crate::data::condition::config::regulator::PropulsionConfig;

pub struct PropulsionRegulator {
    pub velocity: VelocityRegulator,
}

impl PropulsionRegulator {
    pub fn new(config: PropulsionConfig) -> Self {
        Self {
            velocity: VelocityRegulator::new(config.velocity),
        }
    }
}