use super::Hal;
use crate::platform::subsystem::propulsion::NUM_THRUSTERS;

pub struct Peripherals<H: Hal> {
	pub motors: [H::Motor; NUM_THRUSTERS],
	pub camera: H::Camera,
	pub command_transport: H::CommandTransport,
	pub telemetry_transport: H::TelemetryTransport,
	pub video_transport: H::VideoTransport,
}
