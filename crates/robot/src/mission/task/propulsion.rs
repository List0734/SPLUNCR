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
use crate::subsystem::PropulsionSubsystem;
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
		let (emergency_stop, odometry) = {
			let state = self.context.state.read().unwrap();
			(
				state.autonomous.emergency_stop,
				state.perception.navigation.odometry,
			)
		};

		if emergency_stop {
			self.propulsion.set_forces(&[0.0; NUM_THRUSTERS]);
			return;
		}

		let measured_twist = odometry.twist;
		let rotation = odometry.pose.rotation;
		let inv_rotation = rotation.inverse();
		let max = self.propulsion.max_wrench();

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
			PropulsionCommand::OpenLoop(joystick) => {
				Wrench {
					force: joystick.force.component_mul(&max.force),
					torque: joystick.torque.component_mul(&max.torque),
				}
			}
			PropulsionCommand::Stabilized(cmd) => {
				let (_, _, yaw) = rotation.euler_angles();
				let yaw_rotation = nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, yaw);
				let body_force = nalgebra::Vector3::new(
					cmd.wrench.force.x * max.force.x,
					cmd.wrench.force.y * max.force.y,
					0.0,
				);
				let mut world_force = yaw_rotation * body_force.cast::<f64>();

				let heave_force = self.regulator.depth_hold.update(cmd.depth_rate, measured_twist.linear.z as F, dt);
				world_force.z = heave_force as f64;

				let wrench = Wrench {
					force: (inv_rotation * world_force).cast(),
					torque: cmd.wrench.torque.component_mul(&max.torque),
				};
				let compensation = self.propulsion.compensation().compute(&rotation);
				wrench + compensation
			}
		};

		let wrench = if operator_command.auto_level {
			let (roll, pitch, _) = odometry.pose.rotation.euler_angles();
			let (roll_torque, pitch_torque) = self.regulator.auto_level.update(roll as F, pitch as F, dt);
			Wrench {
				force: wrench.force,
				torque: nalgebra::Vector3::new(roll_torque, pitch_torque, wrench.torque.z),
			}
		} else {
			wrench
		};

		let commanded = self.propulsion.allocate(wrench);
		let outputs = self.regulator.thruster.update(&commanded, dt);
		self.propulsion.set_forces(&outputs);

		{
			let mut state = self.context.state.write().unwrap();
			state.action.propulsion.thruster.coast = CoastRegulatorState {
				commanded,
				output: outputs,
			};
		}
	}
}
