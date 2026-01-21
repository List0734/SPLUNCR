use crossbeam::channel::Sender;

use crate::data::transport::telemetry::Message;

#[derive(Clone)]
pub struct Publisher {
    sender: Sender<Message>
}

impl Publisher {
    pub fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    pub fn publish(&self, message: Message) {
        let _ = self.sender.send(message);
    }
}