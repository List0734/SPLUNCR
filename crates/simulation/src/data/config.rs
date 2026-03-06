mod communication;
pub use communication::SimCommunicationConfig;

mod environment;
pub use environment::EnvironmentConfig;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimConfig {
    pub environment: EnvironmentConfig,
    pub communication: SimCommunicationConfig,
}
