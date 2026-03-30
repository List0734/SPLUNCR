mod actuator;
mod body;

pub use actuator::{ActuatorState, PropulsionState};
pub use body::RigidBodyState;

pub struct SimulatorState {
	pub actuators: ActuatorState,
	pub body: RigidBodyState,
}

impl Default for SimulatorState {
	fn default() -> Self {
		Self {
			actuators: ActuatorState::default(),
			body: RigidBodyState::default(),
		}
	}
}
