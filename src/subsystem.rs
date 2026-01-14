mod odometry;
pub use odometry::Odometry;

mod propulsion;
pub use propulsion::Propulsion;

use crate::core::telemetry;

pub trait Subsystem: Send + Sync {
    fn new(telemetry: &telemetry::Publisher) -> Self;

    fn name(&self) -> &str;

    fn telemetry(&self) -> &telemetry::Publisher;
}