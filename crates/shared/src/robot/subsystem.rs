use crate::telemetry;

mod base;
pub use base::Base; 

mod define;

pub trait Subsystem: Send + Sync {
    const NAME: &'static str;

    fn telemetry(&self) -> &telemetry::Publisher;
}