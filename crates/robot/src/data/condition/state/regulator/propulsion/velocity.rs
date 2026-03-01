use serde::{Serialize, Deserialize};
use shared::physics::kinematics::Twist;

use crate::platform::F;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct VelocityRegulatorState {
    pub setpoint: Twist<F>,
    pub output: Twist<F>,
}

impl VelocityRegulatorState {
    pub fn default() -> Self {
        Self {
            setpoint: Twist::zero(),
            output: Twist::zero(),
        }
    }
}
