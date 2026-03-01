pub mod motor;
pub use motor::ZmrEsc;

use crate::{
    hardware::{interface::Hal, peripheral::Peripherals},
    platform::peripheral::MOTOR_PINS,
};

pub struct RpiHal;

impl Hal for RpiHal {
    type Motor = ZmrEsc;
}

impl RpiHal {
    pub fn init() -> Peripherals<Self> {
        Peripherals {
            motors: MOTOR_PINS.map(|pin| ZmrEsc::new(pin).expect("Failed to initialize motor")),
        }
    }
}