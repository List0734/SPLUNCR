pub mod subsystem;
pub mod regulator;

use serde::{Deserialize, Serialize};
pub use subsystem::SubsystemConfig;
pub use regulator::RegulatorConfig;

mod communication;
pub use communication::{CommunicationConfig, VideoConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigBundle {
    pub communication: CommunicationConfig,
    pub subsystem: SubsystemConfig,
    pub regulator: RegulatorConfig,
}

pub trait Config<C> {
    fn update_config(&mut self, config: C);
}
