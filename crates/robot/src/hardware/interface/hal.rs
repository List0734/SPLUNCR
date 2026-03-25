use nalgebra::Vector3;

use framework::hardware::interface::{Camera, Motor, Stream, Datagram, Accelerometer, Gyroscope, Thermometer, Barometer, Bathometer};

use crate::data::config::ConfigBundle;
use crate::platform::F;
use super::Peripherals;

pub trait Hal {
	type Motor: Motor<F>;
	type Camera: Camera;
	type Imu: Accelerometer<Vector3<f32>> + Gyroscope<Vector3<f32>>;
	type AtmosphericSensor: Thermometer<f32> + Barometer<f32>;
	type AquaticSensor: Bathometer<f32> + Barometer<f32> + Thermometer<f32>;
	type CommandTransport: Stream;
	type TelemetryTransport: Datagram;
	type VideoTransport: Datagram;

	fn init(config: &ConfigBundle) -> Peripherals<Self> where Self: Sized;
}
