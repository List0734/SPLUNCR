use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use robot::{Robot, data::condition::{ConfigBundle, RobotCondition}};
use station::Station;

use crate::hal::{SimHal, SimMotor};

pub struct Simulation {
   robot: Arc<Mutex<Robot<SimHal>>>,
   station: Station,
}

impl Simulation {
    pub fn new() -> Self {
        let config = ConfigBundle::load(concat!(env!("CARGO_MANIFEST_DIR"), "/../robot/config.toml"));

        let robot = Arc::new(Mutex::new(Robot::new(config.clone(), SimHal::init())));

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
        loop {
            self.station.run().await;
            sleep(Duration::from_millis(10));
        }
    }
}