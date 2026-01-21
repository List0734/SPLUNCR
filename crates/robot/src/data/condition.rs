pub mod state;
pub use state::StateBundle;

#[derive(Clone, Copy, Debug)]
pub struct RobotCondition {
    pub state: StateBundle,
}

impl RobotCondition {
    pub fn default() -> Self {
        Self {
            state: StateBundle::default(),
        }
    }
}