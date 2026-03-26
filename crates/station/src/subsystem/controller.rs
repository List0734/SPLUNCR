use gilrs::{Axis, Button, GamepadId, Gilrs};
use framework::physics::dynamics::Wrench;
use nalgebra::Vector3;

pub struct Controller {
	gilrs: Gilrs,
	active_gamepad: Option<GamepadId>,
	bidirectional_thrust: bool,
	bidirectional_thrust_prev: bool,
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
		}
	}

	pub fn poll(&mut self, deadband: f32) -> (Wrench<f32>, bool) {
		while let Some(event) = self.gilrs.next_event() {
			if self.active_gamepad.is_none() {
				self.active_gamepad = Some(event.id);
			}
		}

		let Some(id) = self.active_gamepad else {
			return (Wrench::zero(), self.bidirectional_thrust);
		};

		let gamepad = self.gilrs.gamepad(id);
		if !gamepad.is_connected() {
			self.active_gamepad = None;
			return (Wrench::zero(), self.bidirectional_thrust);
		}

		let axis = |a: Axis| -> f32 {
			let v = gamepad.value(a);
			if v.abs() < deadband { 0.0 } else { v }
		};

		let btn = |b: Button| -> f32 { if gamepad.is_pressed(b) { 1.0 } else { 0.0 } };

		let toggle_pressed = gamepad.is_pressed(Button::DPadUp);
		if toggle_pressed && !self.bidirectional_thrust_prev {
			self.bidirectional_thrust = !self.bidirectional_thrust;
		}
		self.bidirectional_thrust_prev = toggle_pressed;

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

		(wrench, self.bidirectional_thrust)
	}
}
