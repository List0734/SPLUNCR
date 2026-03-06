use std::io;

use crate::data::condition::config::CommunicationConfig;

mod socket;
use socket::Udp;

pub struct Communication {
    //pub commands: 
    pub telemetry: Udp,
}

impl Communication {
    pub fn new(config: CommunicationConfig) -> io::Result<Self> {
        Ok(Self {
            telemetry: Udp::new(&config.telemetry.bind_address, &config.telemetry.target_address)?,
        })
    }

    pub fn send_telemetry(&self, data: &[u8]) -> io::Result<()> {
        self.telemetry.send(data)
    }
}