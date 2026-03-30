use std::sync::{Arc, OnceLock, RwLock};
use std::time::Instant;

use nalgebra::{UnitQuaternion, Vector3};

use framework::physics::constants::STANDARD_GRAVITY;
use framework::physics::dynamics::Wrench;
use robot::platform::{Fp, fp};

use crate::data::condition::SimCondition;
use crate::data::config::SimConfig;
use crate::data::context::SimContext;
use crate::data::state::SimulatorState;

static CONDITION: OnceLock<Arc<RwLock<SimCondition>>> = OnceLock::new();

pub struct Simulator {
	context: SimContext,
}

impl Simulator {
	pub fn init_condition(config: SimConfig) -> Arc<RwLock<SimCondition>> {
		CONDITION.get_or_init(|| Arc::new(RwLock::new(SimCondition::new(config)))).clone()
	}

	pub fn shared_condition() -> Arc<RwLock<SimCondition>> {
		CONDITION.get().expect("SimCondition not initialized").clone()
	}

	pub fn new(context: SimContext) -> Self {
		Self { context }
	}

	pub fn run(&self) {
		const RATE_HZ: u64 = 1000;
		let dt = 1.0 / RATE_HZ as Fp;
		let period = std::time::Duration::from_micros(1_000_000 / RATE_HZ);

		loop {
			let start = Instant::now();
			self.step(dt);
			if let Some(remaining) = period.checked_sub(start.elapsed()) {
				std::thread::sleep(remaining);
			}
		}
	}

	fn step(&self, dt: Fp) {
		let mut condition = self.context.condition.write().unwrap();
		let orientation = condition.state.body.pose.rotation;
		let com = Vector3::from(self.context.robot_config.body.mass_properties.center.map(|v| fp(v)));

		let thrust = self.compute_thrust_wrench(&condition.state, &com);
		self.integrate_linear(&mut condition.state, &thrust, orientation, dt);
		self.integrate_angular(&mut condition.state, &thrust, orientation, &com, dt);
	}

	fn compute_thrust_wrench(&self, state: &SimulatorState, com: &Vector3<Fp>) -> Wrench<Fp> {
		let mut wrench = Wrench::zero();

		for (i, thruster) in self.context.robot_config.propulsion.thrusters.iter().enumerate() {
			let duty = fp(state.actuators.propulsion.thruster_duties[i]);
			let max_force = thruster.max_force.unwrap_or(self.context.robot_config.propulsion.default_max_force);
			let magnitude = if duty >= 0.0 {
				duty.powi(2) * fp(max_force.forward)
			} else {
				-(duty.powi(2) * fp(max_force.reverse))
			};

			let direction = Vector3::from(thruster.placement.direction.map(|v| fp(v)));
			let position = Vector3::from(thruster.placement.position.map(|v| fp(v)));

			let force = direction * magnitude;
			wrench.force += force;
			wrench.torque += (position - com).cross(&force);
		}

		wrench
	}

	fn integrate_linear(
		&self,
		state: &mut SimulatorState,
		thrust: &Wrench<Fp>,
		orientation: UnitQuaternion<Fp>,
		dt: Fp,
	) {
		let body = &self.context.robot_config.body;
		let mass = fp(body.mass_properties.mass);

		let thrust_world = orientation * thrust.force;
		let gravity = Vector3::new(0.0, 0.0, -mass * STANDARD_GRAVITY);
		let buoyancy = Vector3::new(0.0, 0.0, fp(body.buoyancy.force));

		let velocity_body = orientation.inverse() * state.body.twist.linear;
		let drag = Vector3::from(body.drag.linear.map(|v| fp(v)));
		let drag_world = orientation * -drag.component_mul(&velocity_body);

		let total_force = thrust_world + gravity + buoyancy + drag_world;
		state.body.acceleration = total_force / mass;
		state.body.twist.linear += state.body.acceleration * dt;
		state.body.pose.translation.vector += state.body.twist.linear * dt;
	}

	fn integrate_angular(
		&self,
		state: &mut SimulatorState,
		thrust: &Wrench<Fp>,
		orientation: UnitQuaternion<Fp>,
		com: &Vector3<Fp>,
		dt: Fp,
	) {
		let body = &self.context.robot_config.body;
		let inertia = Vector3::from(body.mass_properties.inertia.map(|v| fp(v)));

		let buoyancy_center = Vector3::from(body.buoyancy.center.map(|v| fp(v)));
		let buoyancy_body = orientation.inverse() * Vector3::new(0.0, 0.0, fp(body.buoyancy.force));
		let buoyancy_torque = (buoyancy_center - com).cross(&buoyancy_body);

		let drag = Vector3::from(body.drag.angular.map(|v| fp(v)));
		let drag_torque = -drag.component_mul(&state.body.twist.angular);

		let iw = inertia.component_mul(&state.body.twist.angular);
		let gyroscopic = state.body.twist.angular.cross(&iw);

		let total_torque = thrust.torque + buoyancy_torque + drag_torque - gyroscopic;
		let angular_accel = total_torque.component_div(&inertia);
		state.body.twist.angular += angular_accel * dt;
		state.body.pose.rotation *= UnitQuaternion::new(state.body.twist.angular * dt);
	}
}
