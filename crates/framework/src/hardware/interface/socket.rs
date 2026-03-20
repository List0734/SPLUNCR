pub trait Stream {
	type Error: core::fmt::Debug;

	fn try_receive(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error>;
	fn send(&mut self, data: &[u8]) -> Result<(), Self::Error>;
	fn reconnect(&mut self, address: &str) -> Result<(), Self::Error>;
}

pub trait Datagram {
	type Error: core::fmt::Debug;

	fn send(&self, data: &[u8]) -> Result<(), Self::Error>;
	fn try_receive(&self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error>;
	fn set_target(&mut self, address: &str) -> Result<(), Self::Error>;
}
