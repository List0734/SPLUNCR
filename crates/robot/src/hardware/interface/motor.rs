use crate::platform::F;

pub trait Motor {
    type Error: std::fmt::Debug;

    fn init(&mut self) -> Result<(), Self::Error>;

    fn set_duty_cycle(&mut self, duty_cycle: F) -> Result<(), Self::Error>;

    fn set_enabled(&mut self, enabled: bool) -> Result<(), Self::Error>;
}