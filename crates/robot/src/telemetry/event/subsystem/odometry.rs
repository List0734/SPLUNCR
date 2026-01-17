use shared::physics::kinematics::{Pose, Twist};

pub enum Odometry {
    Pose(Pose),
    Twist(Twist),
}