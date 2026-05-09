use gilrs::{Axis, Button, GamepadId, Gilrs};
use framework::physics::dynamics::Wrench;
use nalgebra::Vector3;

pub struct ControllerInput {
	pub wrench: Wrench<f32>,
	pub stabilized: bool,
	pub auto_level: bool,
}

pub struct Controller {
	gilrs: Gilrs,
	active_gamepad: Option<GamepadId>,
	stabilized: bool,
	stabilized_prev: bool,
}

impl Controller {
	pub fn new() -> Self {
		let gilrs = Gilrs::new().expect("Failed to initialize gilrs");
		let active_gamepad = gilrs.gamepads().next().map(|(id, _)| id);
		Self {
			gilrs,
			active_gamepad,
			stabilized: false,
			stabilized_prev: false,
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
				stabilized: self.stabilized,
				auto_level: false,
			};
		};

		let gamepad = self.gilrs.gamepad(id);
		if !gamepad.is_connected() {
			self.active_gamepad = None;
			return ControllerInput {
				wrench: Wrench::zero(),
				stabilized: self.stabilized,
				auto_level: false,
			};
		}

		let axis = |a: Axis| -> f32 {
			let v = gamepad.value(a);
			if v.abs() < deadband { 0.0 } else { v }
		};

		let btn = |b: Button| -> f32 { if gamepad.is_pressed(b) { 1.0 } else { 0.0 } };

		let stabilized_pressed = gamepad.is_pressed(Button::DPadDown);
		if stabilized_pressed && !self.stabilized_prev {
			self.stabilized = !self.stabilized;
		}
		self.stabilized_prev = stabilized_pressed;

		let wrench = Wrench {
			force: Vector3::new(
				-axis(Axis::LeftStickX),
				-axis(Axis::LeftStickY),
				btn(Button::RightTrigger2) - btn(Button::LeftTrigger2),
			),
			torque: Vector3::new(
				axis(Axis::RightStickY),
				btn(Button::LeftTrigger) - btn(Button::RightTrigger),
				-axis(Axis::RightStickX),
			),
		};

		ControllerInput {
			wrench,
			stabilized: self.stabilized,
			auto_level: gamepad.is_pressed(Button::South),
		}
	}
}
