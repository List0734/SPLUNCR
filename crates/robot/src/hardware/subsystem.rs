pub mod propulsion;

pub use propulsion::PropulsionSubsystem;

use crate::data::condition::config::SubsystemConfig;

pub struct Subsystems {
    pub propulsion: PropulsionSubsystem,
}

impl Subsystems {
    pub fn new(config: SubsystemConfig) -> Self {
        Self {
            propulsion: PropulsionSubsystem::new(config.propulsion),
        }
    }
}