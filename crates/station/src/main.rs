use std::sync::{Arc, Mutex};

use robot::data::condition::{ConfigBundle, RobotCondition};
use station::Station;

#[kiss3d::main]
async fn main() {
    let config = ConfigBundle::load(concat!(env!("CARGO_MANIFEST_DIR"), "/../robot/config.toml"));
    let condition = Arc::new(Mutex::new(RobotCondition::default(config)));

    let mut station = Station::new(condition);

    loop {
        station.run().await;
    }
}
