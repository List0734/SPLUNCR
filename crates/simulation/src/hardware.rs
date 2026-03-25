pub mod motor;
pub mod sensor;

pub use motor::SimMotor;
pub use sensor::{SimImu, SimAtmosphericSensor, SimAquaticSensor};

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
	type Imu = SimImu;
	type AtmosphericSensor = SimAtmosphericSensor;
	type AquaticSensor = SimAquaticSensor;
	type CommandTransport = TcpDriver;
	type TelemetryTransport = UdpDriver;
	type VideoTransport = UdpDriver;

	fn init(config: &ConfigBundle) -> Peripherals<Self> {
		let motors = [(); NUM_THRUSTERS].map(|_| SimMotor::new());

		let camera = V4lCamera::new(
			&config.vision.camera.device,
			config.vision.camera.width,
			config.vision.camera.height,
			config.vision.camera.framerate,
		);

		let command_transport = TcpDriver::new(&config.communication.command.listen_address)
			.expect("failed to bind command transport");

		let telemetry_transport = UdpDriver::new(
			&config.communication.telemetry.bind_address,
			&config.communication.telemetry.target_address,
			true,
		).expect("failed to bind telemetry transport");

		let video_transport = UdpDriver::new(
			&config.vision.stream.bind_address,
			&config.vision.stream.target_address,
			false,
		).expect("failed to bind video transport");

		Peripherals {
			motors,
			camera,
			imu: SimImu::new(),
			atmospheric_sensor: SimAtmosphericSensor::new(),
			aquatic_sensor: SimAquaticSensor::new(),
			command_transport,
			telemetry_transport,
			video_transport,
		}
	}
}
