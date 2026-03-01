use serde::{Deserialize, Serialize};

use shared::control::controllers::pid::PIDConfig;

use crate::platform::F;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct VelocityRegulatorConfig {
    pub linear: Linear,
    pub angular: Angular,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Linear {
    pub surge: PIDConfig<F>,
    pub sway: PIDConfig<F>,
    pub heave: PIDConfig<F>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Angular {
    pub roll: PIDConfig<F>,
    pub pitch: PIDConfig<F>,
    pub yaw: PIDConfig<F>,
}