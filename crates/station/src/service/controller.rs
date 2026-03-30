use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use robot::data::command::{DepthHoldCommand, OperatorCommand, PropulsionCommand};

use crate::subsystem::controller::Controller;

use super::context::ServiceContext;

pub struct ControllerService {
	context: ServiceContext,
	controller: Controller,
	poll_period: Duration,
	deadband: f32,
}

impl ControllerService {
	pub fn new(context: ServiceContext, controller: Controller, poll_rate_hz: u32, deadband: f32) -> Self {
		Self {
			context,
			controller,
			poll_period: Duration::from_secs_f64(1.0 / poll_rate_hz as f64),
			deadband,
		}
	}

	pub fn run(mut self) {
		while !self.context.shutdown.load(Ordering::Relaxed) {
			let input = self.controller.poll(self.deadband);

			let propulsion = if input.depth_hold {
				PropulsionCommand::DepthHold(DepthHoldCommand {
					wrench: input.wrench,
					depth_rate: input.wrench.force.z,
				})
			} else {
				PropulsionCommand::OpenLoop(input.wrench)
			};

			*self.context.command.write().unwrap() = OperatorCommand {
				propulsion,
				bidirectional_thrust: input.bidirectional_thrust,
				auto_level: input.auto_level,
			};
			thread::sleep(self.poll_period);
		}
	}
}
