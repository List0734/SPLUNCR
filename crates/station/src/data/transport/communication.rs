use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use shared::data::transport::message::Message;

use crate::data::{config::StationCommunicationConfig, transport::telemetry::Mapper};

mod tcp;
mod udp;
use tcp::Tcp;
use udp::Udp;

pub struct Communication {
    config: StationCommunicationConfig,
    commands: Arc<Mutex<Option<Tcp>>>,
    connected: Arc<AtomicBool>,
    telemetry: Udp,
}

impl Communication {
    pub fn new(config: StationCommunicationConfig) -> io::Result<Self> {
        let telemetry = Udp::new(&config.telemetry.listen_address)?;
        Ok(Self {
            config,
            commands: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
            telemetry,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    pub fn send_command(&self, data: &[u8]) -> io::Result<()> {
        if let Some(tcp) = self.commands.lock().unwrap().as_mut() {
            tcp.send(data)
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))
        }
    }

    pub fn spawn_command_connector(self: &Arc<Self>) -> JoinHandle<()> {
        let commands = Arc::clone(&self.commands);
        let connected = Arc::clone(&self.connected);
        let target_addr = self.config.command.target_address.clone();
        thread::spawn(move || loop {
            if connected.load(Ordering::Acquire) {
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            match Tcp::connect(&target_addr) {
                Ok(tcp) => {
                    *commands.lock().unwrap() = Some(tcp);
                    connected.store(true, Ordering::Release);
                }
                Err(_) => thread::sleep(Duration::from_secs(1)),
            }
        })
    }

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
                    Ok(None) => thread::sleep(Duration::from_millis(1)),
                    Err(e) => eprintln!("Telemetry receive error: {}", e),
                }
            }
        })
    }
}
