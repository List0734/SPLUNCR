pub mod propulsion;
pub use propulsion::PropulsionConfig;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SubsystemConfig {
    pub propulsion: PropulsionConfig,
}