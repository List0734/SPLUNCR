use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

use crate::control::estimator::Odometry;
use crate::control::regulator::PropulsionRegulator;
use crate::data::command::OperatorCommand;
use crate::data::config::ConfigBundle;
use crate::data::state::RobotState;
use crate::hardware::interface::Hal;
use crate::hardware::subsystem::{CommunicationSubsystem, PropulsionSubsystem, VisionSubsystem};
use crate::mission::Mission;
use crate::mission::context::TaskContext;
use crate::mission::task::communication::CommunicationTask;
use crate::mission::task::propulsion::PropulsionTask;
use crate::mission::task::sensor::SensorTask;
use crate::mission::task::vision::VisionTask;

pub struct Robot {
	mission: Mission,
}

impl Robot {
	pub fn new<H: Hal>(config: ConfigBundle) -> Self
	where
		H::Motor: Send + 'static,
		H::Camera: Send + 'static,
		H::CommandTransport: Send + 'static,
		H::TelemetryTransport: Send + 'static,
		H::VideoTransport: Send + 'static,
	{
		// Hardware
		let peripherals = H::init(&config);

		// State
		let state = Arc::new(RwLock::new(RobotState::default()));
		let command = Arc::new(RwLock::new(OperatorCommand::default()));
		let shutdown = Arc::new(AtomicBool::new(false));
		let context = TaskContext::new(state, command, shutdown);

		// Propulsion
		let propulsion = PropulsionSubsystem::new(
			config.propulsion.clone(),
			peripherals.motors,
		);
		let propulsion_regulator = PropulsionRegulator::new(config.propulsion.regulator.clone());
		let propulsion_task = PropulsionTask::new(
			context.clone(),
			propulsion,
			propulsion_regulator,
			config.propulsion.loop_rate_hz,
		);

		// Communication
		let communication = CommunicationSubsystem::new(
			peripherals.command_transport,
			peripherals.telemetry_transport,
		);
		let communication_task = CommunicationTask::new(
			context.clone(),
			communication,
			config.communication.poll_rate_hz,
			config.communication.telemetry.rate_hz,
		);

		// Sensor
		let odometry = Odometry::new();
		let sensor_task = SensorTask::new(
			context.clone(),
			odometry,
			config.sensor.loop_rate_hz,
		);

		// Vision
		let vision = VisionSubsystem::new(
			peripherals.camera,
			peripherals.video_transport,
		);
		let vision_task = VisionTask::new(
			context.clone(),
			vision,
		);

		// Launch
		let tasks: Vec<Box<dyn FnOnce() + Send>> = vec![
			Box::new(move || propulsion_task.run()),
			Box::new(move || communication_task.run()),
			Box::new(move || sensor_task.run()),
			Box::new(move || vision_task.run()),
		];
		let mission = Mission::launch(context, tasks);

		Self { mission }
	}

	pub fn shutdown(self) {
		self.mission.shutdown();
	}
}
