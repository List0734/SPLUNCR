use framework::hardware::interface::{Sensor, Thermometer, Barometer, Bathometer};

pub struct SimAquaticSensor;

impl SimAquaticSensor {
	pub fn new() -> Self {
		Self
	}
}

impl Sensor for SimAquaticSensor {
	type Error = std::convert::Infallible;

	fn calibrate(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl Thermometer<f32> for SimAquaticSensor {
	fn read_temperature(&mut self) -> Result<f32, Self::Error> {
		Ok(15.0)
	}
}

impl Barometer<f32> for SimAquaticSensor {
	fn read_pressure(&mut self) -> Result<f32, Self::Error> {
		Ok(1013.25)
	}
}

impl Bathometer<f32> for SimAquaticSensor {
	fn read_depth(&mut self) -> Result<f32, Self::Error> {
		Ok(0.0)
	}
}
