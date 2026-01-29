use crossbeam::channel::{unbounded, Receiver};

mod message;
pub use message::Message;

mod publisher;
pub use publisher::Publisher;

pub struct Telemetry {
    publisher: Publisher,
    receiver: Receiver<Message>,
}

impl Telemetry {
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

    pub fn receiver(&self) -> Receiver<Message> {
        self.receiver.clone()
    }

    pub fn receive(&self) -> Option<Message> {
        match self.receiver.recv() {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}