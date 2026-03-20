use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};

use framework::hardware::interface::Motor;

use crate::platform::F;

const PERIOD: Duration = Duration::from_micros(20_000); // 50 Hz
const NEUTRAL: Duration = Duration::from_micros(1480); // 7.4% duty cycle
const MIN_PULSE: Duration = Duration::from_micros(1000); // 5% duty cycle, full reverse
const MAX_PULSE: Duration = Duration::from_micros(2000); // 10% duty cycle, full forward
const DEADBAND: F = 0.0125;

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
        if clamped.abs() < DEADBAND {
            return NEUTRAL;
        }
        let half_range = if clamped > 0.0 {
            (MAX_PULSE.as_micros() - NEUTRAL.as_micros()) as F
        } else {
            (NEUTRAL.as_micros() - MIN_PULSE.as_micros()) as F
        };
        let us = NEUTRAL.as_micros() as F + clamped * half_range;
        Duration::from_micros(us as u64)
    }
}

impl Motor<F> for ZmrEsc {
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
