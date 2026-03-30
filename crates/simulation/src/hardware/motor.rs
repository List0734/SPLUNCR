use framework::hardware::interface::Motor;
use robot::platform::F;

use crate::data::context::SimContext;

pub struct SimMotor {
	context: SimContext,
	index: usize,
	enabled: bool,
}

impl SimMotor {
	pub fn new(context: SimContext, index: usize) -> Self {
		Self {
			context,
			index,
			enabled: false,
		}
	}
}

impl Motor<F> for SimMotor {
	type Error = std::convert::Infallible;

	fn init(&mut self) -> Result<(), Self::Error> {
		self.context.condition.write().unwrap().state.actuators.propulsion.thruster_duties[self.index] = 0.0;
		Ok(())
	}

	fn set_duty_cycle(&mut self, duty_cycle: F) -> Result<(), Self::Error> {
		if self.enabled {
			self.context.condition.write().unwrap().state.actuators.propulsion.thruster_duties[self.index] =
				duty_cycle.clamp(-1.0, 1.0);
		}
		Ok(())
	}

	fn set_enabled(&mut self, enabled: bool) -> Result<(), Self::Error> {
		if !enabled {
			self.context.condition.write().unwrap().state.actuators.propulsion.thruster_duties[self.index] = 0.0;
		}
		self.enabled = enabled;
		Ok(())
	}
}
