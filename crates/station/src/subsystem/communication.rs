use std::io::{self, Write};
use std::net::{TcpStream, UdpSocket, SocketAddr};
use std::time::Duration;

use robot::data::command::OperatorCommand;
use robot::data::state::RobotState;

use crate::data::config::StationCommandConfig;

pub struct Communication {
	telemetry: UdpSocket,
	command: Option<TcpStream>,
	target_address: SocketAddr,
}

impl Communication {
	pub fn new(config: &StationCommandConfig, telemetry: UdpSocket) -> Self {
		Self {
			telemetry,
			command: None,
			target_address: config.target_address.parse().unwrap(),
		}
	}

	pub fn is_connected(&self) -> bool {
		self.command.is_some()
	}

	pub fn try_connect(&mut self) -> bool {
		if self.command.is_some() {
			return true;
		}
		match TcpStream::connect_timeout(&self.target_address, Duration::from_millis(500)) {
			Ok(stream) => {
				self.command = Some(stream);
				println!("Connected to robot");
				true
			}
			Err(_) => false,
		}
	}

	pub fn receive_telemetry(&self, buf: &mut [u8]) -> Option<RobotState> {
		match self.telemetry.recv_from(buf) {
			Ok((n, _addr)) => bincode::deserialize::<RobotState>(&buf[..n]).ok(),
			Err(e) if e.kind() == io::ErrorKind::WouldBlock => None,
			Err(e) => {
				eprintln!("Telemetry receive error: {}", e);
				None
			}
		}
	}

	pub fn send_command(&mut self, command: &OperatorCommand) -> io::Result<()> {
		let stream = self.command.as_mut()
			.ok_or_else(|| io::Error::new(io::ErrorKind::NotConnected, "not connected"))?;
		let bytes = bincode::serialize(command).unwrap();
		let len = (bytes.len() as u32).to_be_bytes();

		if let Err(e) = stream.write_all(&len).and_then(|_| stream.write_all(&bytes)) {
			self.command = None;
			return Err(e);
		}
		Ok(())
	}
}
