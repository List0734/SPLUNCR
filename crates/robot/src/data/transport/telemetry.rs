use crossbeam::channel::{unbounded, Receiver};

mod publisher;
pub use publisher::Publisher;

pub mod state;
pub use state::StatePayload;
use shared::data::transport::message::Message;

pub struct Telemetry {
    publisher: Publisher,
    receiver: Receiver<Message<StatePayload>>,
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

    pub fn receiver(&self) -> Receiver<Message<StatePayload>> {
        self.receiver.clone()
    }

    pub fn receive(&self) -> Option<Message<StatePayload>> {
        match self.receiver.try_recv() {
            Ok(message) => Some(message),
            Err(_) => None,
        }
    }
}