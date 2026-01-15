pub use crate::telemetry::{self, Telemetry};

pub use crate::robot::Subsystem;

pub struct Base {
    name: &'static str,
    telemetry: telemetry::Publisher,
}

impl Base {
    pub fn new(telemetry: &Telemetry) -> Self {
        Self {
            name: "",
            telemetry: telemetry.create_publisher("subsystem"),
        }
    }

    pub fn bind<S: Subsystem>(&self) -> Self {
        Self {
            name: S::NAME,
            telemetry: self.telemetry.child(S::NAME),
        }
    }

    pub fn telemetry(&self) -> &telemetry::Publisher {
        &self.telemetry
    }
}
