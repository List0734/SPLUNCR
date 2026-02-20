pub mod propulsion;
pub use propulsion::PropulsionConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubsystemConfig {
    pub propulsion: PropulsionConfig,
}