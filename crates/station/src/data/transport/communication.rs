use std::io;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use shared::data::transport::message::Message;

use crate::data::transport::telemetry::Mapper;

mod udp;
use udp::Udp;

pub struct Communication {
    //pub commands:
    pub telemetry: Udp,
}

impl Communication {
    pub fn new(telemetry_bind_addr: &str) -> io::Result<Self> {
        Ok(Self {
            telemetry: Udp::new(telemetry_bind_addr)?,
        })
    }

    /// Spawn a thread that continuously receives telemetry and updates the robot condition
    pub fn spawn_telemetry_receiver(self: &Arc<Self>, robot: Arc<Mutex<RobotCondition>>) -> JoinHandle<()> {
        let communication = Arc::clone(self);
        thread::spawn(move || {
            let mut buf = [0u8; 65536];
            loop {
                match communication.telemetry.try_receive(&mut buf) {
                    Ok(Some((n, _addr))) => {
                        if let Ok(message) = bincode::deserialize::<Message<State>>(&buf[..n]) {
                            let mut robot = robot.lock().unwrap();
                            Mapper::ingest(&mut robot, message);
                        }
                    }
                    Ok(None) => thread::sleep(std::time::Duration::from_millis(1)),
                    Err(e) => eprintln!("Telemetry receive error: {}", e),
                }
            }
        })
    }
}
