mod value;
use crossbeam::channel::{Receiver, Sender, unbounded};
pub use value::Value;

mod event;
pub use event::Event;

mod publisher;
pub use publisher::Publisher;

pub struct Telemetry {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

impl Telemetry {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();

        Self {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn sender(&self) -> &Sender<Event> {
        &self.sender
    }

    pub fn receiver(&self) -> &Receiver<Event> {
        &self.receiver
    }

    pub fn create_publisher(&self, base_path: &'static str) -> Publisher {
        Publisher::new(self.sender.clone(), base_path)
    }
}