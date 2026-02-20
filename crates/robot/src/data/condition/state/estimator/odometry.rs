use serde::Serialize;
use shared::physics::kinematics::{Pose, Twist};

use crate::platform::Fp;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct OdometryEstimatorState {
    pub pose: Pose<Fp>,
    pub twist: Twist<Fp>,
}

impl OdometryEstimatorState {
    pub fn default() -> Self {
        Self {
            pose: Pose::identity(),
            twist: Twist::zero(),
        }
    }
}