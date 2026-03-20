use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use robot::data::command::{OperatorCommand, PropulsionCommand};

use crate::subsystem::controller::Controller;

use super::context::StationContext;

pub struct ControllerService {
	context: StationContext,
	controller: Controller,
	poll_period: Duration,
	deadband: f32,
}

impl ControllerService {
	pub fn new(context: StationContext, controller: Controller, poll_rate_hz: u32, deadband: f32) -> Self {
		Self {
			context,
			controller,
			poll_period: Duration::from_secs_f64(1.0 / poll_rate_hz as f64),
			deadband,
		}
	}

	pub fn run(mut self) {
		while !self.context.shutdown.load(Ordering::Relaxed) {
			let wrench = self.controller.poll(self.deadband);
			*self.context.command.write().unwrap() = OperatorCommand {
				propulsion: PropulsionCommand::OpenLoop(wrench),
			};
			thread::sleep(self.poll_period);
		}
	}
}
