use std::sync::{Arc, Mutex};

use robot::data::config::RobotConfig;
use framework::data::config::load_config;
use robot::data::condition::RobotCondition;
use station::{Station, data::config::StationConfig};

#[kiss3d::main]
async fn main() {
    let robot_config: RobotConfig = load_config(concat!(env!("CARGO_MANIFEST_DIR"), "/../robot/config.toml"));
    let station_config: StationConfig = load_config(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));
    let robot_condition = Arc::new(Mutex::new(RobotCondition::new(robot_config)));

    let mut station = Station::new(robot_condition, station_config);

    loop {
        station.run().await;
    }
}
