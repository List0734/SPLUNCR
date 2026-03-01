use serde::{Deserialize, Serialize};

mod velocity;
pub use velocity::VelocityConfig;

pub mod thruster;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PropulsionConfig {
    pub velocity: VelocityConfig,
    pub thruster: thruster::CoastConfig,
}