use serde::{Deserialize, Serialize};

mod velocity;
pub use velocity::VelocityConfig;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PropulsionConfig {
    pub velocity: VelocityConfig,
}