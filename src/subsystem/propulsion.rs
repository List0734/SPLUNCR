use na::Isometry3;

use crate::core::physics::kinematics::Pose;

mod thruster;
pub use thruster::{
    Thruster,
};

pub struct Propulsion {
    pose: Pose,
    thrusters: [Thruster; 8],
}

/*
impl Propulsion {
    pub fn new(thuster_poses: [Pose; 8]) -> self {
        Self {
            pose: Pose::identity(),
            thrusters: 
        }
    }

    pub fn get_thruster_positions(&self) -> [Pose; 8] {
        self.thrusters.iter().map(|t| t.pose).collect();
    }
}
    */