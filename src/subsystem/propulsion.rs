use crate::{core::{physics::kinematics::Pose, telemetry}, define_subsystem, subsystem::{Base, Subsystem}};

mod thruster;
use na::{Vector3, UnitQuaternion};
pub use thruster::{
    Thruster,
};

define_subsystem!(
    Propulsion,
    "propulsion",
    { 
        pose: Pose,
        thrusters: [Thruster; 8]
    }
);

impl Propulsion {
    pub fn init(base: Base) -> Self {
        let thruster_poses: [Pose; 8] = std::array::from_fn(|i| match i {
            0 => Pose::from_parts(Vector3::new(5.0, 5.0, 5.0).into(),
                                UnitQuaternion::from_euler_angles(0.631914, 0.212930, -0.631914).into()),
            1 => Pose::from_parts(Vector3::new(-5.0, 5.0, 5.0).into(),
                                UnitQuaternion::from_euler_angles(0.631914, -0.212930, 0.631914).into()),
            2 => Pose::from_parts(Vector3::new(-5.0, -5.0, 5.0).into(),
                                UnitQuaternion::from_euler_angles(1.219917, -0.908651, 1.219917).into()),
            3 => Pose::from_parts(Vector3::new(5.0, -5.0, 5.0).into(),
                                UnitQuaternion::from_euler_angles(1.219917, 0.908651, -1.219917).into()),
            4 => Pose::from_parts(Vector3::new(5.0, 5.0, -5.0).into(),
                                UnitQuaternion::from_euler_angles(-0.631914, -0.212930, -0.631914).into()),
            5 => Pose::from_parts(Vector3::new(-5.0, 5.0, -5.0).into(),
                                UnitQuaternion::from_euler_angles(-0.631914, 0.212930, 0.631914).into()),
            6 => Pose::from_parts(Vector3::new(-5.0, -5.0, -5.0).into(),
                                UnitQuaternion::from_euler_angles(-1.219917, 0.908651, 1.219917).into()),
            7 => Pose::from_parts(Vector3::new(5.0, -5.0, -5.0).into(),
                                UnitQuaternion::from_euler_angles(-1.219917, -0.908651, -1.219917).into()),
            _ => unreachable!(),
        });

        let thrusters = std::array::from_fn(|i| Thruster::new(thruster_poses[i]));

        Self {
            base,
            pose: Pose::identity(),
            thrusters,
        }
    }

    pub fn test_telemetry(&self) {
        self.telemetry().publish("test", 1.0);
    }
    
    pub fn thruster_positions(&self) -> [Pose; 8] {
        std::array::from_fn(|i| self.thrusters[i].pose())
    }
}