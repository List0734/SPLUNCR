use std::sync::{Arc, Mutex};
use std::thread;

use robot::{Robot, data::{config::RobotConfig, condition::RobotCondition}};
use station::{Station, data::config::StationConfig};

use crate::data::config::SimConfig;
use crate::data::context::SimContext;
use crate::hardware::SimHal;
use crate::simulator::Simulator;

pub struct Simulation {
	robot: Robot,
	station: Station,
}

impl Simulation {
	pub fn new(robot_config: RobotConfig, station_config: StationConfig, simulation_config: SimConfig) -> Self {
		let condition = Simulator::init_condition(simulation_config);
		let sim_context = SimContext::new(condition, robot_config.clone());

		let simulator = Simulator::new(sim_context);
		thread::spawn(move || simulator.run());

		let robot_condition = Arc::new(Mutex::new(RobotCondition::new(robot_config.clone())));
		let robot = Robot::new::<SimHal>(robot_config);
		let station = Station::new(Arc::clone(&robot_condition), station_config);

		Self { robot, station }
	}

	pub async fn run(&mut self) {
		self.station.run().await;
	}

	pub fn shutdown(self) {
		self.station.shutdown();
		self.robot.shutdown();
	}
}
