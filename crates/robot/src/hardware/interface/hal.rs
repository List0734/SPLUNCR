use framework::hardware::interface::{Camera, Motor, Stream, Datagram};

use crate::data::config::ConfigBundle;
use crate::platform::F;
use super::Peripherals;

pub trait Hal {
	type Motor: Motor<F>;
	type Camera: Camera;
	type CommandTransport: Stream;
	type TelemetryTransport: Datagram;
	type VideoTransport: Datagram;

	fn init(config: &ConfigBundle) -> Peripherals<Self> where Self: Sized;
}
