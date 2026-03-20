use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use robot::Robot;
use robot::data::config::ConfigBundle;
use robot::hardware::driver::RpiHal;
use framework::data::config::load_config;

fn main() {
	let mut config_path = env::current_exe().expect("cannot get exe path");
	config_path.pop();
	config_path.push("config.toml");

	let config: ConfigBundle = load_config(config_path.to_str().expect("invalid config path"));
	println!("configuration loaded from {:?}", config_path);

	let robot = Robot::new::<RpiHal>(config);

	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();
	ctrlc::set_handler(move || r.store(false, Ordering::Relaxed))
		.expect("failed to set ctrl-c handler");

	while running.load(Ordering::Relaxed) {
		std::thread::sleep(std::time::Duration::from_millis(100));
	}

	robot.shutdown();
}
