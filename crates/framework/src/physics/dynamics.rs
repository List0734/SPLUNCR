use nalgebra::{RealField, Vector3, Vector6};
use serde::{Serialize, Deserialize};

pub type Force<S> = S;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Wrench<S: RealField + Serialize + Copy> {
    pub force: Vector3<S>,
    pub torque: Vector3<S>,
}

impl<S> Wrench<S>
where
    S: RealField + Serialize + Copy,
{
    pub fn zero() -> Self {
        Self {
            force: Vector3::zeros(),
            torque: Vector3::zeros(),
        }
    }

    pub fn as_vector(&self) -> Vector6<S> {
        Vector6::new(
            self.force.x,
            self.force.y,
            self.force.z,
            self.torque.x,
            self.torque.y,
            self.torque.z,
        )
    }
}