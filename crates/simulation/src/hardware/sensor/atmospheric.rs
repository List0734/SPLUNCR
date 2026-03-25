use framework::hardware::interface::{Sensor, Thermometer, Barometer};

pub struct SimAtmosphericSensor;

impl SimAtmosphericSensor {
	pub fn new() -> Self {
		Self
	}
}

impl Sensor for SimAtmosphericSensor {
	type Error = std::convert::Infallible;

	fn calibrate(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl Thermometer<f32> for SimAtmosphericSensor {
	fn read_temperature(&mut self) -> Result<f32, Self::Error> {
		Ok(25.0)
	}
}

impl Barometer<f32> for SimAtmosphericSensor {
	fn read_pressure(&mut self) -> Result<f32, Self::Error> {
		Ok(101325.0)
	}
}
