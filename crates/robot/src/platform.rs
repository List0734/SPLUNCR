// Low precision float type
pub type F = f32;

// High precision float type
pub type Fp = f64;
// High precision promotion helper
pub fn fp(x: F) -> Fp { x as Fp }

pub mod subsystem {
    pub mod propulsion {
        pub const NUM_THRUSTERS: usize = 8;
    }
}

pub mod peripheral {
    pub const MOTOR_PINS: [u8; super::subsystem::propulsion::NUM_THRUSTERS] = [
        4, 5, 6, 12, 13, 16, 17, 18,
    ];
}

