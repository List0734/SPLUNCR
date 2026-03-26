use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use framework::hardware::interface::Motor;
use framework::physics::dynamics::Wrench;
use framework::physics::kinematics::Twist;

use crate::control::regulator::PropulsionRegulator;
use crate::data::state::action::propulsion::velocity::VelocityRegulatorState;
use crate::data::state::action::propulsion::thruster::coast::CoastRegulatorState;
use crate::data::command::PropulsionCommand;
use crate::hardware::subsystem::PropulsionSubsystem;
use crate::mission::context::TaskContext;
use crate::platform::{F, subsystem::propulsion::NUM_THRUSTERS};

pub struct PropulsionTask<M: Motor<F>> {
	context: TaskContext,
	propulsion: PropulsionSubsystem<M>,
	regulator: PropulsionRegulator,
	period: Duration,
}

impl<M: Motor<F>> PropulsionTask<M> {
	pub fn new(
		context: TaskContext,
		propulsion: PropulsionSubsystem<M>,
		regulator: PropulsionRegulator,
		rate_hz: u32,
	) -> Self {
		Self {
			context,
			propulsion,
			regulator,
			period: Duration::from_secs_f64(1.0 / rate_hz as f64),
		}
	}

	pub fn run(mut self) {
		self.propulsion.init();

		let mut last = Instant::now();

		while !self.context.shutdown.load(Ordering::Relaxed) {
			let now = Instant::now();
			let dt = now.duration_since(last).as_secs_f32();
			last = now;

			self.step(dt);

			let elapsed = now.elapsed();
			if elapsed < self.period {
				thread::sleep(self.period - elapsed);
			}
		}

		self.propulsion.stop();
	}

	fn step(&mut self, dt: F) {
		let operator_command = self.context.command.read().unwrap().clone();
		let command = operator_command.propulsion;
		let (emergency_stop, measured_twist) = {
			let state = self.context.state.read().unwrap();
			(
				state.autonomous.emergency_stop,
				state.perception.navigation.odometry.twist,
			)
		};

		if emergency_stop {
			self.propulsion.set_duty_cycles(&[0.0; NUM_THRUSTERS]);
			return;
		}

		let wrench = match command {
			PropulsionCommand::Velocity(setpoint) => {
				self.regulator.velocity.set_setpoint(setpoint);
				let measured: Twist<F> = Twist {
					linear: measured_twist.linear.cast(),
					angular: measured_twist.angular.cast(),
				};
				let output = self.regulator.velocity.update(&measured, dt);
				let mut state = self.context.state.write().unwrap();
				state.action.propulsion.velocity = VelocityRegulatorState {
					setpoint,
					output,
				};
				Wrench { force: output.linear, torque: output.angular }
			}
			PropulsionCommand::OpenLoop(wrench) => wrench,
		};
		let commanded = self.propulsion.allocate(wrench, operator_command.bidirectional_thrust);
		let outputs = self.regulator.thruster.update(&commanded, dt);
		self.propulsion.set_duty_cycles(&outputs);

		{
			let mut state = self.context.state.write().unwrap();
			state.action.propulsion.thruster.coast = CoastRegulatorState {
				commanded,
				output: outputs,
			};
		}
	}
}
