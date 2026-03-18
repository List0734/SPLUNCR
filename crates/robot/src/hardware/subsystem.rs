pub mod propulsion;
pub mod vision;

pub use propulsion::PropulsionSubsystem;
pub use vision::VisionSubsystem;

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