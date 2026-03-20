use std::{io, net::UdpSocket};

use framework::hardware::interface::Datagram;

pub struct UdpDriver {
	socket: UdpSocket,
	target_addr: String,
}

impl UdpDriver {
	pub fn new(bind_addr: &str, target_addr: &str) -> io::Result<Self> {
		let socket = UdpSocket::bind(bind_addr)?;
		socket.set_nonblocking(true)?;

		Ok(Self {
			socket,
			target_addr: target_addr.to_string(),
		})
	}
}

impl Datagram for UdpDriver {
	type Error = io::Error;

	fn send(&self, data: &[u8]) -> Result<(), Self::Error> {
		self.socket.send_to(data, &self.target_addr)?;
		Ok(())
	}

	fn try_receive(&self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error> {
		match self.socket.recv_from(buf) {
			Ok((n, _addr)) => Ok(Some(n)),
			Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
			Err(e) => Err(e),
		}
	}

	fn set_target(&mut self, address: &str) -> Result<(), Self::Error> {
		self.target_addr = address.to_string();
		Ok(())
	}
}
