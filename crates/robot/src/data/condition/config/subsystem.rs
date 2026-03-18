pub mod propulsion;
pub use propulsion::PropulsionConfig;

pub mod vision;
pub use vision::VisionConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubsystemConfig {
    pub propulsion: PropulsionConfig,
    pub vision: VisionConfig,
}