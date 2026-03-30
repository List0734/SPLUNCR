use super::config::SimConfig;
use super::state::SimulatorState;

pub struct SimCondition {
	pub config: SimConfig,
	pub state: SimulatorState,
}

impl SimCondition {
	pub fn new(config: SimConfig) -> Self {
		Self {
			config,
			state: SimulatorState::default(),
		}
	}
}
