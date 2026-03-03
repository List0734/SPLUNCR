pub mod motor;
pub use motor::ZmrEsc;

use crate::{
    hardware::{interface::{Hal, motor::Motor}, peripheral::Peripherals},
    platform::peripheral::MOTOR_PINS,
};

pub struct RpiHal;

impl Hal for RpiHal {
    type Motor = ZmrEsc;
}

impl RpiHal {
    pub fn init() -> Peripherals<Self> {
        Peripherals {
            motors: MOTOR_PINS.map(|pin| {
                let mut motor = ZmrEsc::new(pin).expect("Failed to initialize motor");
                motor.init().expect("Failed to init motor PWM");
                motor.set_enabled(true).expect("Failed to enable motor");
                motor
            }),
        }
    }
}