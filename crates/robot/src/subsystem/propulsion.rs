use shared::physics::kinematics::Pose;
use shared::define_subsystem;
use shared::robot::subsystem::{Subsystem, Base};
use shared::telemetry;


mod thruster;
use nalgebra::{DMatrix, UnitQuaternion, Vector3};
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
        let thruster_poses: [Pose; 8] = std::array::from_fn(|i| {
            let pos = match i {
                0 => Vector3::new(5.0, 5.0, 5.0),
                1 => Vector3::new(-5.0, 5.0, 5.0),
                2 => Vector3::new(-5.0, -5.0, 5.0),
                3 => Vector3::new(5.0, -5.0, 5.0),
                4 => Vector3::new(5.0, 5.0, -5.0),
                5 => Vector3::new(-5.0, 5.0, -5.0),
                6 => Vector3::new(-5.0, -5.0, -5.0),
                7 => Vector3::new(5.0, -5.0, -5.0),
                _ => unreachable!(),
            };

            // Compute rotation to point away from origin (cube center)
            let forward = pos.normalize(); // direction away from center
            let up = Vector3::y_axis();    // or choose another stable up vector
            let rotation = UnitQuaternion::face_towards(&forward, &up);

            Pose::from_parts(pos.into(), rotation.into())
        });

        /*
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
        */

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

    pub fn compute_thruster_forces(&self, desired_force: Vector3<f32>) -> Vec<f32> {
        let n = self.thrusters.len();
        let mut a = DMatrix::<f32>::zeros(3, n);

        // Fill matrix with thruster direction vectors
        for (i, thruster) in self.thrusters.iter().enumerate() {
            a.column_mut(i).copy_from(&thruster.direction());
        }

        // Solve Ax = desired_force using least-squares pseudo-inverse
        let x = a.pseudo_inverse(1e-6).unwrap() * desired_force;

        // Clamp each thruster output to [0, max_force]
        self.thrusters
            .iter()
            .enumerate()
            .map(|(i, t)| x[i].clamp(0.0, 100.0))
            .collect()
    }
    
    pub fn thruster_positions(&self) -> [Pose; 8] {
        std::array::from_fn(|i| self.thrusters[i].pose())
    }
}