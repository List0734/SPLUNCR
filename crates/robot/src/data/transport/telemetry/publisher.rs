use std::time::{SystemTime, UNIX_EPOCH};

use crossbeam::channel::Sender;

use shared::data::transport::message::Message;

use crate::data::transport::telemetry::StatePayload;

#[derive(Clone)]
pub struct Publisher {
    sender: Sender<Message<StatePayload>>
}

impl Publisher {
    pub fn new(sender: Sender<Message<StatePayload>>) -> Self {
        Self { sender }
    }

    pub fn publish(&self, payload: StatePayload) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time is before UNIX_EPOCH")
            .as_micros() as u64;

        let message = Message {
            timestamp,
            payload,
        };
        
        let _ = self.sender.send(message);
    }
}