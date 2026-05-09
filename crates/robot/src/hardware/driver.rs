#[cfg(any(feature = "camera", feature = "picamera"))]
pub mod camera;

#[cfg(feature = "rpi")]
pub mod motor;

#[cfg(feature = "i2c")]
pub mod sensor;

#[cfg(feature = "network")]
pub mod socket;

#[cfg(feature = "rpi")]
use crate::{
	data::config::RobotConfig,
	hardware::interface::{Hal, Peripherals},
};

#[cfg(feature = "rpi")]
pub struct RpiHal;

#[cfg(feature = "rpi")]
impl Hal for RpiHal {
	type Motor = motor::ZmrEsc;
	type Camera = camera::PiCamera;
	type Imu = sensor::Mpu6500;
	type AtmosphericSensor = sensor::Bmp280;
	type AquaticSensor = sensor::Ms5837;
	type CommandTransport = socket::TcpDriver;
	type TelemetryTransport = socket::UdpDriver;
	type VideoTransport = socket::UdpDriver;

	fn init(config: &RobotConfig) -> Peripherals<Self> {
		let motors = config.propulsion.thrusters
			.map(|thruster| {
				motor::ZmrEsc::new(thruster.gpio_pin).expect("failed to initialize motor")
			});

		let camera = camera::PiCamera::new(
			&config.vision.camera.device,
			config.vision.camera.width,
			config.vision.camera.height,
			config.vision.camera.framerate,
			config.vision.camera.flip_vertical,
			config.vision.camera.flip_horizontal,
		);

		let command_transport = socket::TcpDriver::new(&config.communication.command.listen_address)
			.expect("failed to bind command transport");

		let telemetry_transport = socket::UdpDriver::new(
			&config.communication.telemetry.bind_address,
			&config.communication.telemetry.target_address,
			true,
		).expect("failed to bind telemetry transport");

		let video_transport = socket::UdpDriver::new(
			&config.vision.stream.bind_address,
			&config.vision.stream.target_address,
			false,
		).expect("failed to bind video transport");

		let imu = sensor::Mpu6500::new().expect("failed to initialize IMU");

		let atmospheric_sensor = sensor::Bmp280::new().expect("failed to initialize atmospheric sensor");

		let aquatic_sensor = sensor::Ms5837::new().expect("failed to initialize depth sensor");

		Peripherals {
			motors,
			camera,
			imu,
			atmospheric_sensor,
			aquatic_sensor,
			command_transport,
			telemetry_transport,
			video_transport,
		}
	}
}
