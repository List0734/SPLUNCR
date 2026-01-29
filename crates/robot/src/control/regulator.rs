pub mod propulsion;
pub use propulsion::Propulsion;

use crate::data::condition::config::RegulatorConfig;

pub struct Regulators {
    pub propulsion: Propulsion,
}

impl Regulators {
    pub fn new(config: RegulatorConfig) -> Self {
        Self {
            propulsion: Propulsion::new(config.propulsion),
        }
    }
}