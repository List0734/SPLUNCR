use crate::{
    hardware::interface::Hal,
    platform::subsystem::propulsion::NUM_THRUSTERS,
};

pub struct Peripherals<H: Hal> {
    pub motors: [H::Motor; NUM_THRUSTERS],
}
