use na::Isometry3;

use crate::core::physics::kinematics::Pose;

pub struct Thruster {
    pose: Pose,
    pub thrust: f32,
}

impl Thruster {
    pub fn new(pose: Pose) -> Self {
        Self {
            pose: pose,
            thrust: 0.0,
        }
    } 

    pub fn command_thrust(mut self, thrust: f32) {
        self.thrust = thrust;
    }
}