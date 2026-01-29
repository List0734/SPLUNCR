use shared::physics::kinematics::{Pose, Twist};

#[derive(Clone, Copy, Debug)]
pub struct OdometryState {
    pub pose: Pose,
    pub twist: Twist,
}

impl OdometryState {
    pub fn default() -> Self {
        Self {
            pose: Pose::identity(),
            twist: Twist::zero(),
        }
    }
}