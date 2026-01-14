use crate::core::{physics::kinematics::Pose, telemetry};

mod thruster;
pub use thruster::{
    Thruster,
};

pub struct Propulsion {
    telemetry: telemetry::Publisher,
    pose: Pose,
    thrusters: [Thruster; 8],
}

impl Propulsion {
    pub fn new(telemetry: telemetry::Publisher, thruster_poses: [Pose; 8]) -> Self {
        let thrusters = std::array::from_fn(|i| Thruster::new(thruster_poses[i]));

        Self {
            telemetry,
            pose: Pose::identity(),
            thrusters,
        }
    }
    
    pub fn thruster_positions(&self) -> [Pose; 8] {
        std::array::from_fn(|i| self.thrusters[i].pose())
    }
}