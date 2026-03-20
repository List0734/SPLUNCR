use std::{
	io::{self, Read, Write},
	net::{TcpListener, TcpStream},
};

use framework::hardware::interface::Stream;

pub struct TcpDriver {
	listener: TcpListener,
	stream: Option<TcpStream>,
}

impl TcpDriver {
	pub fn new(bind_addr: &str) -> io::Result<Self> {
		let listener = TcpListener::bind(bind_addr)?;
		listener.set_nonblocking(true)?;

		Ok(Self {
			listener,
			stream: None,
		})
	}
}

impl Stream for TcpDriver {
	type Error = io::Error;

	fn try_receive(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error> {
		if self.stream.is_none() {
			match self.listener.accept() {
				Ok((stream, _addr)) => {
					stream.set_nonblocking(true)?;
					self.stream = Some(stream);
				}
				Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(None),
				Err(e) => return Err(e),
			}
		}

		let stream = self.stream.as_mut().unwrap();
		let mut len_buf = [0u8; 4];
		match stream.read_exact(&mut len_buf) {
			Ok(()) => {}
			Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(None),
			Err(e) => {
				self.stream = None;
				return Err(e);
			}
		}

		let len = u32::from_be_bytes(len_buf) as usize;
		stream.set_nonblocking(false)?;
		let result = stream.read_exact(&mut buf[..len]);
		stream.set_nonblocking(true)?;
		match result {
			Ok(()) => Ok(Some(len)),
			Err(e) => {
				self.stream = None;
				Err(e)
			}
		}
	}

	fn send(&mut self, data: &[u8]) -> Result<(), Self::Error> {
		if let Some(stream) = &mut self.stream {
			let len = (data.len() as u32).to_be_bytes();
			stream.write_all(&len)?;
			stream.write_all(data)?;
			stream.flush()?;
			Ok(())
		} else {
			Err(io::Error::new(io::ErrorKind::NotConnected, "no client connected"))
		}
	}

	fn reconnect(&mut self, address: &str) -> Result<(), Self::Error> {
		self.stream = None;
		self.listener = TcpListener::bind(address)?;
		self.listener.set_nonblocking(true)?;
		Ok(())
	}
}
