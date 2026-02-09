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

