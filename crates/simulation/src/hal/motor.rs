use robot::{hardware::interface::motor::Motor, platform::F};

pub struct SimMotor {
    duty_cycle: F,
    enabled: bool,
}

impl SimMotor {
    pub fn new() -> Self {
        Self {
            duty_cycle: 0.0,
            enabled: false,
        }
    }

    pub fn duty_cycle(&self) -> F {
        self.duty_cycle
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Motor for SimMotor {
    type Error = std::convert::Infallible;

    fn init(&mut self) -> Result<(), Self::Error> {
        self.duty_cycle = 0.0;
        Ok(())
    }

    fn set_duty_cycle(&mut self, duty_cycle: F) -> Result<(), Self::Error> {
        if self.enabled {
            self.duty_cycle = duty_cycle.clamp(-1.0, 1.0);
        }
        Ok(())
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<(), Self::Error> {
        if !enabled {
            self.duty_cycle = 0.0;
        }
        self.enabled = enabled;
        Ok(())
    }
}
