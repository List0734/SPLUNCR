use nalgebra::{Isometry3, Vector3};

pub type Pose = Isometry3<f32>;

pub struct Twist {
    pub linear: Vector3<f32>,
    pub angular: Vector3<f32>,
}

impl Twist {
    pub fn zero() -> Self {
        Self {
            linear: Vector3::zeros(),
            angular: Vector3::zeros(),
        }
    }
}