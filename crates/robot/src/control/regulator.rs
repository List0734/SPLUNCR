pub mod propulsion;
pub use propulsion::PropulsionRegulator;

use crate::data::condition::config::RegulatorConfig;

pub struct Regulators {
    pub propulsion: PropulsionRegulator,
}

impl Regulators {
    pub fn new(config: RegulatorConfig) -> Self {
        Self {
            propulsion: PropulsionRegulator::new(config.propulsion),
        }
    }
}