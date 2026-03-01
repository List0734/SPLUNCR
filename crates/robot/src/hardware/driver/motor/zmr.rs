use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

use crate::{hardware::interface::motor::Motor, platform::F};

const PERIOD: Duration = Duration::from_micros(20_000); // 50 Hz
const NEUTRAL: Duration = Duration::from_micros(1500);
const MIN_PULSE: Duration = Duration::from_micros(1100); // full reverse
const MAX_PULSE: Duration = Duration::from_micros(1900); // full forward

pub struct ZmrEsc {
    pin: OutputPin,
    enabled: bool,
}

impl ZmrEsc {
    pub fn new(gpio_pin: u8) -> Result<Self, rppal::gpio::Error> {
        let pin = Gpio::new()?.get(gpio_pin)?.into_output_low();
        Ok(Self {
            pin,
            enabled: false,
        })
    }

    fn duty_cycle_to_pulse_width(duty_cycle: F) -> Duration {
        let clamped = duty_cycle.clamp(-1.0, 1.0);
        let half_range = (MAX_PULSE.as_micros() - NEUTRAL.as_micros()) as F;
        let us = NEUTRAL.as_micros() as F + clamped * half_range;
        Duration::from_micros(us as u64)
    }
}

impl Motor for ZmrEsc {
    type Error = rppal::gpio::Error;

    fn init(&mut self) -> Result<(), Self::Error> {
        self.pin.set_pwm(PERIOD, NEUTRAL)?;
        Ok(())
    }

    fn set_duty_cycle(&mut self, duty_cycle: F) -> Result<(), Self::Error> {
        if self.enabled {
            let pulse = Self::duty_cycle_to_pulse_width(duty_cycle);
            self.pin.set_pwm(PERIOD, pulse)?;
        }
        Ok(())
    }

    fn set_enabled(&mut self, enabled: bool) -> Result<(), Self::Error> {
        if enabled && !self.enabled {
            self.pin.set_pwm(PERIOD, NEUTRAL)?;
        } else if !enabled && self.enabled {
            self.pin.clear_pwm()?;
        }
        self.enabled = enabled;
        Ok(())
    }
}
