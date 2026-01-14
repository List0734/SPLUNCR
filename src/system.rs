use std::any::type_name;
use std::sync::{Arc, RwLock};

use crate::core::telemetry::Telemetry;
use crate::subsystem::{Subsystem, Context};

mod subsystems;
use subsystems::Subsystems;

pub struct System {
    telemetry: Telemetry,
    subsystems: Subsystems,
}

impl System {
    pub fn new() -> Self {
        let telemetry = Telemetry::new();
        let subsystems = Subsystems::new();
        
        Self {
            telemetry,
            subsystems, 
        }
    }
    
    pub fn telemetry(&self) -> &Telemetry {
        &self.telemetry
    }
    
    // Create subsystem with automatic telemetry injection
    pub fn create_subsystem<S>(&self) -> Arc<S>
    where
        S: Subsystem + 'static,
        S: From<Context>,
    {
        // Automatically grab name from subsystem type (e.g. "A::B::C" -> "C")
        let type_name = type_name::<S>();
        let name = type_name.rsplit("::").next().unwrap();
        
        // Create publisher with the type name as base path
        let publisher = self.telemetry.create_publisher(name);
        let context = Context::new(publisher);
        
        // Construct the subsystem using From<Context>
        let subsystem = Arc::new(S::from(context));
        
        // Cast to Arc<dyn Subsystem> and register
        let subsystem_dyn: Arc<dyn Subsystem> = subsystem.clone();
        self.subsystems.write().unwrap().push(subsystem_dyn);
        
        subsystem
    }
}