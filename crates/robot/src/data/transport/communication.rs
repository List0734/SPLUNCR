use std::io;

use crate::data::condition::config::CommunicationConfig;

pub mod command;
mod socket;
use socket::{Tcp, Udp};

pub struct Communication {
    pub commands: Tcp,
    pub telemetry: Udp,
}

impl Communication {
    pub fn new(config: CommunicationConfig) -> io::Result<Self> {
        Ok(Self {
            commands: Tcp::new(&config.command.listen_address)?,
            telemetry: Udp::new(&config.telemetry.bind_address, &config.telemetry.target_address)?,
        })
    }

    pub fn send_telemetry(&self, data: &[u8]) -> io::Result<()> {
        self.telemetry.send(data)
    }
}
