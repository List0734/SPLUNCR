use robot::platform::F;
use robot::platform::subsystem::propulsion::NUM_THRUSTERS;

pub struct ActuatorState {
	pub propulsion: PropulsionState,
}

pub struct PropulsionState {
	pub thruster_duties: [F; NUM_THRUSTERS],
}

impl Default for ActuatorState {
	fn default() -> Self {
		Self {
			propulsion: PropulsionState {
				thruster_duties: [0.0; NUM_THRUSTERS],
			},
		}
	}
}
