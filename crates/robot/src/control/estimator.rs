mod odometry;
pub use odometry::Odometry;

use crate::data::transport::telemetry::Publisher;

pub struct Estimators {
    pub odometry: Odometry,
}

impl Estimators {
    pub fn new(telemetry: Publisher) -> Self {
        Self {
            odometry: Odometry::new(telemetry),
        }
    }
}