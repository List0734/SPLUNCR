pub mod decode;
mod receiver;

use std::net::UdpSocket;

pub use decode::Decoder;
pub use receiver::RawFrame;

use receiver::FragmentReceiver;

pub struct Video {
	receiver: FragmentReceiver,
}

impl Video {
	pub fn new(socket: UdpSocket) -> Self {
		Self {
			receiver: FragmentReceiver::new(socket),
		}
	}

	pub fn try_receive(&mut self, buf: &mut [u8]) -> Option<RawFrame> {
		self.receiver.try_receive(buf)
	}
}
