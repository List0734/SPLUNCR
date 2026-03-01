use robot::{Robot, data::condition::ConfigBundle, hardware::driver::RpiHal};
use std::{env, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

fn main() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("Program started at {}", since_epoch.as_secs());

    let mut exe_path = env::current_exe().expect("cannot get exe path");
    exe_path.pop();
    exe_path.push("config.toml");

    let config = ConfigBundle::load(&exe_path);
    println!("Configuration loaded from {:?}", exe_path);

    let mut robot = Robot::new(config, RpiHal::init());

    loop {
        robot.run();

        thread::sleep(Duration::from_millis(100));
    }
}