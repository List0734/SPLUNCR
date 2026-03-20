pub mod motor;
pub use motor::SimMotor;

use robot::{
	data::config::ConfigBundle,
	hardware::{
		interface::{Hal, Peripherals},
		driver::{
			camera::V4lCamera,
			socket::{TcpDriver, UdpDriver},
		},
	},
	platform::subsystem::propulsion::NUM_THRUSTERS,
};

pub struct SimHal;

impl Hal for SimHal {
	type Motor = SimMotor;
	type Camera = V4lCamera;
	type CommandTransport = TcpDriver;
	type TelemetryTransport = UdpDriver;
	type VideoTransport = UdpDriver;

	fn init(config: &ConfigBundle) -> Peripherals<Self> {
		let motors = [(); NUM_THRUSTERS].map(|_| SimMotor::new());

		let camera = V4lCamera::new(
			&config.vision.camera.device,
			config.vision.camera.width,
			config.vision.camera.height,
		);

		let command_transport = TcpDriver::new(&config.communication.command.listen_address)
			.expect("failed to bind command transport");

		let telemetry_transport = UdpDriver::new(
			&config.communication.telemetry.bind_address,
			&config.communication.telemetry.target_address,
		).expect("failed to bind telemetry transport");

		let video_transport = UdpDriver::new(
			&config.vision.stream.bind_address,
			&config.vision.stream.target_address,
		).expect("failed to bind video transport");

		Peripherals {
			motors,
			camera,
			command_transport,
			telemetry_transport,
			video_transport,
		}
	}
}
