use std::sync::Arc;

use crate::{core::telemetry::Telemetry, subsystem::{Context, Propulsion, Subsystem}};

pub struct Subsystems {
    propulsion: Arc<Propulsion>,
}

impl Subsystems {
    pub fn new() -> Self {
        Self {
            propulsion: System
        }
    }

    fn create_subsystem<S>(telemetry: &Telemetry) -> Arc<S>
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
}