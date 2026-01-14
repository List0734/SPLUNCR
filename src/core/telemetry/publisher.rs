use std::{collections::HashMap, sync::{Arc, Mutex}};
use crossbeam::channel::{Sender, Receiver, unbounded};

use super::{Value, Event};

// Thread-safe telemetry publisher (to be injected into subsystems/commands)
#[derive(Clone)]
pub struct Publisher {
    sender: Sender<Event>,
    // Base path for this publisher (e.g., "propulsion")
    base_path: String,

    // Cache for computed full paths
    path_cache: Arc<Mutex<HashMap<&'static str, String>>>,
}

impl Publisher {
    pub fn new(sender: Sender<Event>, base_path: &'static str) -> Self {
        Self {
            sender,
            base_path: base_path.to_string(),
            path_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Publish telemetry with a relative key
    pub fn publish<V: Into<Value>>(&self, key: &'static str, value: V) {
        // Lookup or compute full path
        let full_path = {
            let mut cache = self.path_cache.lock().unwrap();
            if let Some(path) = cache.get(key) {
                path.clone()
            } else {
                let path = if key.is_empty() {
                    self.base_path.clone()
                } else {
                    format!("{}.{}", self.base_path, key)
                };
                cache.insert(key, path.clone());
                path
            }
        };

        let event = Event::new(full_path, value.into());
        let _ = self.sender.send(event); // drop on backpressure
    }
    
    // Create a child publisher with extended path
    pub fn child(&self, name: &'static str) -> Self {
        let new_base = format!("{}.{}", self.base_path, name);

        Self {
            sender: self.sender.clone(),
            base_path: new_base,
            path_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}