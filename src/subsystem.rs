mod odometry;
pub use odometry::Odometry;

mod propulsion;
pub use propulsion::Propulsion;

use crate::core::telemetry;

pub trait Subsystem: Send + Sync {
    fn name(&self) -> &str;

    fn telemetry(&self) -> &telemetry::Publisher;
}

pub struct Context {
    telemetry: telemetry::Publisher,
}

impl Context {
    pub fn new(telemetry: telemetry::Publisher) -> Self {
        Self { telemetry }
    }

    pub fn telemetry(&self) -> &telemetry::Publisher {
        &self.telemetry
    }
}

    pub fn create_subsystem<S>(telemetry: &Telemetry) -> Arc<S>
    where
        S: Subsystem + 'static,
        S: From<Context>,
    {
        let type_name = std::any::type_name::<S>();
        let name = type_name.rsplit("::").next().unwrap();
        
        let publisher = telemetry.create_publisher(name);
        let context = Context::new(publisher);
        
        Arc::new(S::from(context))
    }