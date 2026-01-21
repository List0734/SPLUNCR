mod odometry;
pub use odometry::OdometryState;

#[derive(Clone, Copy, Debug)]
pub struct StateBundle {
    pub odometry: OdometryState,
}

impl StateBundle {
    pub fn default() -> Self {
        Self {
            odometry: OdometryState::default(),
        }
    }
}