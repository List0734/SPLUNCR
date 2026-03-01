use robot::{Robot, hardware::driver::RpiHal};
use std::{thread, time::{Duration, SystemTime, UNIX_EPOCH}};

fn main() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("Program started at {}", since_epoch.as_secs());

    let mut robot = Robot::new(RpiHal::init());

    loop {
        robot.run();

        thread::sleep(Duration::from_millis(100));
    }
}