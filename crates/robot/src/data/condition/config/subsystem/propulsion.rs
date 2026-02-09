use serde::Deserialize;

mod thruster;
pub use thruster::ThrusterConfig;

use crate::platform::subsystem::propulsion::NUM_THRUSTERS;

#[derive(Clone, Debug, Deserialize)]
pub struct PropulsionConfig {
    pub thrusters: [ThrusterConfig; NUM_THRUSTERS]
}