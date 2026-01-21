use std::{sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};

use kiss3d;

use robot::{Robot, data::{transport::telemetry::Message, condition::{RobotCondition, StateBundle}}};
use station::Station;

//mod simulation;
//use simulation::Simulation;

#[kiss3d::main]
async fn main() {
    println!("Simulation started");

    let mut robot = Arc::new(Mutex::new(Robot::new()));

    let condition = Arc::new(Mutex::new(RobotCondition::default()));

    let mut station = Station::new(condition);

    /*
    loop {
        robot.run();

        while let Some(message) = robot.telemetry().receive() {
            station.receive_message(message);
        }

        station.run().await;

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    */

    /*
    let telemetry = robot.telemetry(); // contains publisher & receiver
    let receiver = telemetry.receiver(); // clone the receiver
    */

    let robot_clone = Arc::clone(&robot);

    let receiver = {
        let robot_guard = robot.lock().unwrap();
        let telemetry = robot_guard.telemetry();
        telemetry.receiver()
    };

    // Spawn robot in a separate thread
    thread::spawn(move || {
        loop {
            let mut robot_guard = robot_clone.lock().unwrap();
            robot_guard.run();
            drop(robot_guard);

            std::thread::sleep(Duration::from_millis(10));
        }
    });

    // Async station loop
    loop {
        // non-blocking receive all available messages
        /*
        while let Ok(message) = receiver.try_recv() {
            station.receive_message(message);
        }
        */

        // async station logic
        station.run(&receiver).await;

        sleep(Duration::from_millis(10));
    }
}