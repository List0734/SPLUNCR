pub mod state;
pub use state::StateBundle;

pub mod config;
pub use config::ConfigBundle;

#[derive(Clone, Copy, Debug)]
pub struct RobotCondition {
    pub config: ConfigBundle,
    pub state: StateBundle,
}

impl RobotCondition {
    pub fn default(config: ConfigBundle) -> Self {
        Self {
            config,
            state: StateBundle::default(),
        }
    }
}