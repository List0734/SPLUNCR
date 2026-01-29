use serde::Deserialize;

use shared::control::controllers::pid::PIDConfig;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct VelocityConfig {
    pub linear: Linear,
    pub angular: Angular,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Linear {
    pub surge: PIDConfig,
    pub sway: PIDConfig,
    pub heave: PIDConfig,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Angular {
    pub roll: PIDConfig,
    pub pitch: PIDConfig,
    pub yaw: PIDConfig,
}