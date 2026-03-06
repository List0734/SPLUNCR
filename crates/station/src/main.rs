use std::sync::{Arc, Mutex};

use robot::data::condition::{ConfigBundle, RobotCondition};
use shared::data::config::load_config;
use station::{Station, data::config::StationConfig};

#[kiss3d::main]
async fn main() {
    let robot_config: ConfigBundle = load_config(concat!(env!("CARGO_MANIFEST_DIR"), "/../robot/config.toml"));
    let station_config: StationConfig = load_config(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));
    let condition = Arc::new(Mutex::new(RobotCondition::default(robot_config)));

    let mut station = Station::new(condition, station_config);

    loop {
        station.run().await;
    }
}
