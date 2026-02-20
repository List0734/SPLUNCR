use serde::{Deserialize, Serialize};

pub mod propulsion;
pub use propulsion::PropulsionConfig;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct RegulatorConfig {
    pub propulsion: PropulsionConfig,
}