use kiss3d;

use robot::data::condition::ConfigBundle;
use shared::data::config::load_config;
use simulation::{Simulation, data::config::SimConfig};

#[kiss3d::main]
async fn main() {
    println!("Simulation started");

    let robot_config: ConfigBundle = load_config(concat!(env!("CARGO_MANIFEST_DIR"), "/../robot/config.toml"));
    let sim_config: SimConfig = load_config(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));

    let mut simulation = Simulation::new(robot_config, sim_config);
    simulation.spawn_robot_thread();
    simulation.run_station_loop().await;
}
