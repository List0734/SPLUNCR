use std::sync::{Arc, RwLock};

use robot::data::config::RobotConfig;

use super::condition::SimCondition;

pub struct SimContext {
	pub condition: Arc<RwLock<SimCondition>>,
	pub robot_config: RobotConfig,
}

impl SimContext {
	pub fn new(condition: Arc<RwLock<SimCondition>>, robot_config: RobotConfig) -> Self {
		Self { condition, robot_config }
	}

	pub fn clone(&self) -> Self {
		Self {
			condition: Arc::clone(&self.condition),
			robot_config: self.robot_config.clone(),
		}
	}
}
