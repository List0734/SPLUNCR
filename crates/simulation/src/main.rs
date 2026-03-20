use robot::data::config::ConfigBundle;
use station::data::config::StationConfig;
use framework::data::config::{load_raw, load_with_overrides};
use simulation::{Simulation, data::config::SimConfig};

#[kiss3d::main]
async fn main() {
	let simulation_config = load_raw(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));

	let robot_config: ConfigBundle = load_with_overrides(
		concat!(env!("CARGO_MANIFEST_DIR"), "/../robot/config.toml"),
		simulation_config.get("overrides").and_then(|o| o.get("robot")),
	);

	let station_config: StationConfig = load_with_overrides(
		concat!(env!("CARGO_MANIFEST_DIR"), "/../station/config.toml"),
		simulation_config.get("overrides").and_then(|o| o.get("station")),
	);

	let simulation_config: SimConfig = simulation_config.try_into()
		.expect("failed to deserialize simulation config");

	let mut simulation = Simulation::new(robot_config, station_config, simulation_config);

	println!("Simulation started");

	loop {
		simulation.run().await;
	}
}
