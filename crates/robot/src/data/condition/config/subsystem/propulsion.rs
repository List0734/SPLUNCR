use serde::{Deserialize, Serialize};

mod thruster;
pub use thruster::ThrusterConfig;

use crate::platform::subsystem::propulsion::NUM_THRUSTERS;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct PropulsionConfig {
    pub thrusters: [ThrusterConfig; NUM_THRUSTERS]
}