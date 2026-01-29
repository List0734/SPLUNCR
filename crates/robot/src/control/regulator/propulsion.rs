mod velocity;
pub use velocity::Velocity;

use crate::data::condition::config::regulator::PropulsionConfig;

pub struct Propulsion {
    pub velocity: Velocity,
}

impl Propulsion {
    pub fn new(config: PropulsionConfig) -> Self {
        Self {
            velocity: Velocity::new(config.velocity),
        }
    }
}