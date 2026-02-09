use nalgebra::{Isometry3, RealField, Unit, Vector3, Vector6};

pub type Distance<S> = S;
pub type Velocity<S> = S;
pub type Acceleration<S> = S;

pub type Pose<S> = Isometry3<S>;
#[derive(Clone, Copy, Debug)]
pub struct Placement<S: RealField> {
    pub position: Vector3<S>,
    pub direction: Unit<Vector3<S>>,
}

impl<S: RealField> Placement<S> {
    pub fn new(position: Vector3<S>, direction: Unit<Vector3<S>>) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn from_arrays(position: [S; 3], direction: [S; 3]) -> Self {
        Self {
            position: Vector3::from(position),
            direction: Unit::new_normalize(Vector3::from(direction)),
        }
    }

    pub fn zero() -> Self {
        Self {
            position: Vector3::zeros(),
            direction: Unit::new_normalize(Vector3::x()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Twist<S> {
    pub linear: Vector3<S>,
    pub angular: Vector3<S>,
}

impl<S: RealField> Twist<S> {
    pub fn zero() -> Self {
        Self {
            linear: Vector3::zeros(),
            angular: Vector3::zeros(),
        }
    }
}