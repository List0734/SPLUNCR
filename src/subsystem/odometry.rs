use na::Vector3;

use crate::{core::{physics::kinematics::{Pose, Twist}, telemetry}, define_subsystem};
use crate::subsystem::{Base, Subsystem};

define_subsystem!(
    Odometry,
    "odometry",
    {
        pose: Pose,
        twist: Twist,
    }
);

impl Odometry {
    pub fn init(base: Base) -> Self {
        Self {
            base,
            pose: Pose::identity(),
            twist: Twist::zero(),
        }
    }

    pub fn pose(&self) -> &Pose {
        &self.pose
    }

    pub fn twist(&self) -> &Twist {
        &self.twist
    }

    pub fn integrate(&mut self, dt: f32) {
        let delta = Pose::new(
            self.twist.linear * dt,
            self.twist.angular * dt,
        );

        self.pose *= delta;
    }

    pub fn apply_linear_acceleration(&mut self, acceleration: Vector3<f32>, dt: f32) {
        self.twist.linear += acceleration * dt;
    }

    pub fn update_angular_velocity(&mut self, velocity: Vector3<f32>) {
        self.twist.angular = velocity;
    }
}