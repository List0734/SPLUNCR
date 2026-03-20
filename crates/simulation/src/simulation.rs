use std::sync::{Arc, Mutex};

use robot::{Robot, data::config::ConfigBundle};
use station::{Station, data::{condition::RobotCondition, config::StationConfig}};

use crate::data::config::SimConfig;
use crate::hardware::SimHal;

pub struct Simulation {
	robot: Robot,
	station: Station,
}

impl Simulation {
	pub fn new(robot_config: ConfigBundle, station_config: StationConfig, simulation_config: SimConfig) -> Self {
		let condition = Arc::new(Mutex::new(RobotCondition::new(robot_config.clone())));
		let robot = Robot::new::<SimHal>(robot_config);
		let station = Station::new(Arc::clone(&condition), station_config);

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
