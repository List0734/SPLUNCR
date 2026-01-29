use serde::Deserialize;

pub mod propulsion;
pub use propulsion::PropulsionConfig;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct RegulatorConfig {
    pub propulsion: PropulsionConfig,
}