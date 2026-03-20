pub struct Frame<'a> {
	pub data: &'a [u8],
	pub width: u16,
	pub height: u16,
}

pub trait Camera {
	type Error: core::fmt::Debug;

	fn capture(&mut self) -> Result<Frame<'_>, Self::Error>;
}
