use super::config::RobotConfig;
use super::state::RobotState;

#[derive(Clone, Debug)]
pub struct RobotCondition {
	pub config: RobotConfig,
	pub state: RobotState,
}

impl RobotCondition {
	pub fn new(config: RobotConfig) -> Self {
		Self {
			config,
			state: RobotState::default(),
		}
	}
}
