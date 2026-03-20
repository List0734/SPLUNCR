use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use crate::data::condition::RobotCondition;
use crate::data::config::StationConfig;
use crate::service::Services;
use crate::service::communication::CommunicationService;
use crate::service::context::StationContext;
use crate::service::controller::ControllerService;
use crate::service::video::VideoService;
use crate::subsystem::communication::Communication;
use crate::subsystem::controller::Controller;
use crate::subsystem::gui::Gui;
use crate::subsystem::gui::scene::ConnectingScene;
use crate::subsystem::video::Video;

pub struct Station {
	context: StationContext,
	services: Services,
	gui: Gui,
}

impl Station {
	pub fn new(condition: Arc<Mutex<RobotCondition>>, config: StationConfig) -> Self {
		// Context
		let context = StationContext::new(condition);

		// Sockets
		let telemetry_socket = UdpSocket::bind(&config.communication.telemetry.listen_address)
			.expect("Failed to bind telemetry socket");
		telemetry_socket.set_nonblocking(true).unwrap();

		let video_socket = UdpSocket::bind(&config.communication.video.listen_address)
			.expect("Failed to bind video socket");
		video_socket.set_nonblocking(true).unwrap();

		// Communication
		let communication = Communication::new(&config.communication.command, telemetry_socket);
		let communication_service = CommunicationService::new(
			context.clone(),
			communication,
			&config.communication,
		);

		// Video
		let video = Video::new(video_socket);
		let video_service = VideoService::new(
			context.clone(),
			video,
			config.communication.poll_rate_hz,
		);

		// Controller
		let controller = Controller::new();
		let controller_service = ControllerService::new(
			context.clone(),
			controller,
			config.controller.poll_rate_hz,
			config.controller.deadband,
		);

		// Services
		let tasks: Vec<Box<dyn FnOnce() + Send>> = vec![
			Box::new(move || communication_service.run()),
			Box::new(move || video_service.run()),
			Box::new(move || controller_service.run()),
		];
		let services = Services::launch(context.clone(), tasks);

		// GUI
		let initial_scene = ConnectingScene::new(Arc::clone(&context.state));
		let robot_snapshot = context.condition.lock().unwrap().clone();
		let gui = Gui::new(initial_scene, &robot_snapshot, Arc::clone(&context.video_frame));

		Self { context, services, gui }
	}

	pub async fn run(&mut self) {
		self.gui.run(Arc::clone(&self.context.condition)).await;
	}

	pub fn shutdown(self) {
		self.services.shutdown();
	}
}
