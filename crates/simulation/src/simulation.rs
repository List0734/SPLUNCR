use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use robot::{Robot, data::condition::{ConfigBundle, RobotCondition}};
use station::Station;

pub struct Simulation {
   robot: Arc<Mutex<Robot>>,
   station: Station, 
}

impl Simulation {
    pub fn new() -> Self {
        let robot = Arc::new(Mutex::new(Robot::new()));
        
        let config = ConfigBundle::load("../robot/config.toml");
        let condition = Arc::new(Mutex::new(RobotCondition::default(config)));
        let station = Station::new(condition);

        Self { robot, station }
    }

    pub fn spawn_robot_thread(&self) {
        let robot_clone = Arc::clone(&self.robot);
        thread::spawn(move || {
            loop {
                {
                    let mut robot_guard = robot_clone.lock().unwrap();
                    robot_guard.run();
                }

                thread::sleep(Duration::from_millis(10));
            }
        });
    }

    pub async fn run_station_loop(&mut self) {
        let receiver = self.robot.lock().unwrap().telemetry().receiver();

        loop {
            self.station.run(&receiver).await;
            sleep(Duration::from_millis(10));
        }
    }
}