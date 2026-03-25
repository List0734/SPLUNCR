use super::Hal;
use crate::platform::subsystem::propulsion::NUM_THRUSTERS;

pub struct Peripherals<H: Hal> {
	pub motors: [H::Motor; NUM_THRUSTERS],
	pub camera: H::Camera,
	pub imu: H::Imu,
	pub atmospheric_sensor: H::AtmosphericSensor,
	pub aquatic_sensor: H::AquaticSensor,
	pub command_transport: H::CommandTransport,
	pub telemetry_transport: H::TelemetryTransport,
	pub video_transport: H::VideoTransport,
}
