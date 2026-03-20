use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

use crate::data::command::OperatorCommand;
use crate::data::state::RobotState;

pub struct TaskContext {
	pub state: Arc<RwLock<RobotState>>,
	pub command: Arc<RwLock<OperatorCommand>>,
	pub shutdown: Arc<AtomicBool>,
}

impl TaskContext {
	pub fn new(
		state: Arc<RwLock<RobotState>>,
		command: Arc<RwLock<OperatorCommand>>,
		shutdown: Arc<AtomicBool>,
	) -> Self {
		Self { state, command, shutdown }
	}

	pub fn clone(&self) -> Self {
		Self {
			state: Arc::clone(&self.state),
			command: Arc::clone(&self.command),
			shutdown: Arc::clone(&self.shutdown),
		}
	}
}
