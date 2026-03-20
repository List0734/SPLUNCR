use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, RwLock};

use robot::data::command::OperatorCommand;

use crate::data::condition::RobotCondition;
use crate::data::state::StationState;
use crate::data::video::VideoFrame;

pub struct StationContext {
	pub condition: Arc<Mutex<RobotCondition>>,
	pub state: Arc<RwLock<StationState>>,
	pub command: Arc<RwLock<OperatorCommand>>,
	pub video_frame: Arc<Mutex<Option<VideoFrame>>>,
	pub shutdown: Arc<AtomicBool>,
}

impl StationContext {
	pub fn new(condition: Arc<Mutex<RobotCondition>>) -> Self {
		Self {
			condition,
			state: Arc::new(RwLock::new(StationState::default())),
			command: Arc::new(RwLock::new(OperatorCommand::default())),
			video_frame: Arc::new(Mutex::new(None)),
			shutdown: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn clone(&self) -> Self {
		Self {
			condition: Arc::clone(&self.condition),
			state: Arc::clone(&self.state),
			command: Arc::clone(&self.command),
			video_frame: Arc::clone(&self.video_frame),
			shutdown: Arc::clone(&self.shutdown),
		}
	}
}
