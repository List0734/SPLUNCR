use crossbeam::channel::{unbounded, Receiver};

use robot::data::transport::communication::command::Command;
use shared::data::transport::message::Message;

mod publisher;
pub use publisher::Publisher;

pub struct Commands {
    publisher: Publisher,
    receiver: Receiver<Message<Command>>,
}

impl Commands {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self {
            publisher: Publisher::new(tx),
            receiver: rx,
        }
    }

    pub fn publisher(&self) -> Publisher {
        self.publisher.clone()
    }

    pub fn receive(&self) -> Option<Message<Command>> {
        match self.receiver.try_recv() {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}
