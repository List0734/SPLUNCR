use serde::Deserialize;

mod velocity;
pub use velocity::VelocityConfig;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct PropulsionConfig {
    pub velocity: VelocityConfig,
}