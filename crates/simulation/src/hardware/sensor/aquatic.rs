use framework::hardware::interface::{Sensor, Thermometer, Barometer, Bathometer};
use framework::physics::constants::STANDARD_GRAVITY;

use crate::data::context::SimContext;

pub struct SimAquaticSensor {
	context: SimContext,
}

impl SimAquaticSensor {
	pub fn new(context: SimContext) -> Self {
		Self { context }
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
		let condition = self.context.condition.read().unwrap();
		Ok(condition.config.environment.water_temperature)
	}
}

impl Barometer<f32> for SimAquaticSensor {
	fn read_pressure(&mut self) -> Result<f32, Self::Error> {
		let condition = self.context.condition.read().unwrap();
		let depth = -condition.state.body.pose.translation.vector.z as f32;
		let hydrostatic = condition.config.environment.water_density * STANDARD_GRAVITY as f32 * depth;
		Ok(condition.config.environment.surface_pressure + hydrostatic)
	}
}

impl Bathometer<f32> for SimAquaticSensor {
	fn read_depth(&mut self) -> Result<f32, Self::Error> {
		let condition = self.context.condition.read().unwrap();
		let depth = -condition.state.body.pose.translation.vector.z as f32;
		Ok(depth)
	}
}
