use robot::{Robot, data::condition::ConfigBundle, hardware::driver::RpiHal};
use shared::data::config::load_config;
use std::{env, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

fn main() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("Program started at {}", since_epoch.as_secs());

    let mut config_path = env::current_exe().expect("cannot get exe path");
    config_path.pop();
    config_path.push("config.toml");

    let config_path_str = config_path.to_str().expect("invalid config path");
    let config: ConfigBundle = load_config(config_path_str);
    println!("Configuration loaded from {:?}", config_path);

    let mut robot = Robot::new(config, RpiHal::init());

    robot.init_motors();
    thread::sleep(Duration::from_millis(3000));

    loop {
        robot.run();

        thread::sleep(Duration::from_millis(10));
    }
}
