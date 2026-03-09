use std::time::{SystemTime, UNIX_EPOCH};

use crossbeam::channel::Sender;

use robot::data::transport::communication::command::Command;
use shared::data::transport::message::Message;

#[derive(Clone)]
pub struct Publisher {
    sender: Sender<Message<Command>>,
}

impl Publisher {
    pub fn new(sender: Sender<Message<Command>>) -> Self {
        Self { sender }
    }

    pub fn publish(&self, payload: Command) {
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
