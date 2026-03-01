pub mod motor;
pub use motor::SimMotor;

use robot::{
    hardware::{interface::Hal, peripheral::Peripherals},
    platform::subsystem::propulsion::NUM_THRUSTERS,
};

pub struct SimHal;

impl Hal for SimHal {
    type Motor = SimMotor;
}

impl SimHal {
    pub fn init() -> Peripherals<Self> {
        Peripherals {
            motors: [(); NUM_THRUSTERS].map(|_| SimMotor::new()),
        }
    }
}
