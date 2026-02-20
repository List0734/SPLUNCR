mod odometry;
pub use odometry::OdometryEstimatorState;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
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