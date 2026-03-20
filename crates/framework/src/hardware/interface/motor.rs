pub trait Motor<S> {
	type Error: core::fmt::Debug;

	fn init(&mut self) -> Result<(), Self::Error>;
	fn set_duty_cycle(&mut self, duty_cycle: S) -> Result<(), Self::Error>;
	fn set_enabled(&mut self, enabled: bool) -> Result<(), Self::Error>;
}
