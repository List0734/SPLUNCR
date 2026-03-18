use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use robot::{Robot, data::condition::{ConfigBundle, RobotCondition}, hardware::subsystem::VisionSubsystem};
use station::{Station, data::config::StationConfig};

use crate::data::config::SimConfig;
use crate::hal::{SimHal, SimMotor};

pub struct Simulation {
   robot: Arc<Mutex<Robot<SimHal>>>,
   station: Station,
}

impl Simulation {
    pub fn new(mut robot_config: ConfigBundle, sim_config: SimConfig) -> Self {
        robot_config.communication = sim_config.communication.robot;

        let station_config = StationConfig {
            communication: sim_config.communication.station,
        };

        let mut vision = VisionSubsystem::new(
            robot_config.subsystem.vision.camera.clone(),
            robot_config.communication.video.clone(),
        );
        thread::spawn(move || {
            loop {
                if let Err(e) = vision.capture_and_send() {
                    eprintln!("Vision error: {}", e);
                }
            }
        });

        let robot = Arc::new(Mutex::new(Robot::new(robot_config.clone(), SimHal::init())));

        let condition = Arc::new(Mutex::new(RobotCondition::default(robot_config)));
        let station = Station::new(condition, station_config);

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
