pub mod propulsion;
pub use propulsion::PropulsionRegulator;

use crate::{data::condition::config::RegulatorConfig, data::transport::telemetry::Publisher};

pub struct Regulators {
    pub propulsion: PropulsionRegulator,
}

impl Regulators {
    pub fn new(config: RegulatorConfig, telemetry: Publisher) -> Self {
        Self {
            propulsion: PropulsionRegulator::new(config.propulsion, telemetry),
        }
    }
}