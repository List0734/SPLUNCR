use gilrs::{Axis, Button, GamepadId, Gilrs};
use framework::physics::dynamics::Wrench;
use nalgebra::Vector3;

pub struct ControllerInput {
	pub wrench: Wrench<f32>,
	pub bidirectional_thrust: bool,
	pub depth_hold: bool,
	pub auto_level: bool,
}

pub struct Controller {
	gilrs: Gilrs,
	active_gamepad: Option<GamepadId>,
	bidirectional_thrust: bool,
	bidirectional_thrust_prev: bool,
	depth_hold: bool,
	depth_hold_prev: bool,
}

impl Controller {
	pub fn new() -> Self {
		let gilrs = Gilrs::new().expect("Failed to initialize gilrs");
		let active_gamepad = gilrs.gamepads().next().map(|(id, _)| id);
		Self {
			gilrs,
			active_gamepad,
			bidirectional_thrust: false,
			bidirectional_thrust_prev: false,
			depth_hold: false,
			depth_hold_prev: false,
		}
	}

	pub fn poll(&mut self, deadband: f32) -> ControllerInput {
		while let Some(event) = self.gilrs.next_event() {
			if self.active_gamepad.is_none() {
				self.active_gamepad = Some(event.id);
			}
		}

		let Some(id) = self.active_gamepad else {
			return ControllerInput {
				wrench: Wrench::zero(),
				bidirectional_thrust: self.bidirectional_thrust,
				depth_hold: self.depth_hold,
				auto_level: false,
			};
		};

		let gamepad = self.gilrs.gamepad(id);
		if !gamepad.is_connected() {
			self.active_gamepad = None;
			return ControllerInput {
				wrench: Wrench::zero(),
				bidirectional_thrust: self.bidirectional_thrust,
				depth_hold: self.depth_hold,
				auto_level: false,
			};
		}

		let axis = |a: Axis| -> f32 {
			let v = gamepad.value(a);
			if v.abs() < deadband { 0.0 } else { v }
		};

		let btn = |b: Button| -> f32 { if gamepad.is_pressed(b) { 1.0 } else { 0.0 } };

		let bidirectional_pressed = gamepad.is_pressed(Button::DPadUp);
		if bidirectional_pressed && !self.bidirectional_thrust_prev {
			self.bidirectional_thrust = !self.bidirectional_thrust;
		}
		self.bidirectional_thrust_prev = bidirectional_pressed;

		let depth_hold_pressed = gamepad.is_pressed(Button::DPadDown);
		if depth_hold_pressed && !self.depth_hold_prev {
			self.depth_hold = !self.depth_hold;
		}
		self.depth_hold_prev = depth_hold_pressed;

		let wrench = Wrench {
			force: Vector3::new(
				-axis(Axis::LeftStickY),
				axis(Axis::LeftStickX),
				btn(Button::RightTrigger2) - btn(Button::LeftTrigger2),
			),
			torque: Vector3::new(
				btn(Button::LeftTrigger) - btn(Button::RightTrigger),
				axis(Axis::RightStickY),
				-axis(Axis::RightStickX),
			),
		};

		ControllerInput {
			wrench,
			bidirectional_thrust: self.bidirectional_thrust,
			depth_hold: self.depth_hold,
			auto_level: gamepad.is_pressed(Button::South),
		}
	}
}
