mod odometry;
pub use odometry::OdometryEstimatorState;

#[derive(Clone, Copy, Debug)]
pub struct EstimatorBundle {
    pub odometry: OdometryEstimatorState,
}

impl EstimatorBundle {
    pub fn default() -> Self {
        Self {
            odometry: OdometryEstimatorState::default(),
        }
    }
}