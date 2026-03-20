use robot::data::{config::ConfigBundle, state::RobotState};

#[derive(Clone, Debug)]
pub struct RobotCondition {
	pub config: ConfigBundle,
	pub state: RobotState,
}

impl RobotCondition {
	pub fn new(config: ConfigBundle) -> Self {
		Self {
			config,
			state: RobotState::default(),
		}
	}
}
