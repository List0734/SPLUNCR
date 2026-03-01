use serde::{Deserialize, Serialize};

use crate::platform::F;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct CoastConfig {
    pub accel_rate: F,
    pub decel_rate: F,
}
