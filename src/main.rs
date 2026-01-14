extern crate kiss3d;
extern crate nalgebra as na;

use std::thread::sleep;
use std::time;

use crossbeam::channel::RecvTimeoutError;
use std::time::Duration;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::event::{Event, WindowEvent, Key, Action};
use na::{Isometry3, Rotation3, UnitQuaternion};
use na::{Point3, Translation3, Vector3};

mod subsystem;
mod core;

use subsystem::{
    Odometry,
    Propulsion,
};

mod system;
use system::System;

use core::physics::kinematics::Pose;

use std::path::Path;

use crate::core::telemetry::Telemetry;

#[kiss3d::main]
async fn main() {

    let telemetry = Telemetry::new();
    
    let publisher = telemetry.create_publisher("propulsion");

    publisher.publish("rpm", 12);

    let thruster_positions = [
        Pose::from_parts(Vector3::new(1.0, 1.0, 1.0).into(), Rotation3::from_euler_angles(std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, 0.0).into()),
    ];
    //let propulsion: Propulsion = Propulsion::new(thruster_positions);

    let mut odometry: Odometry = Odometry::new();

    let mut system = System::new();

    system.create_subsystem();

    println!("Program started");

    // Setup kiss3d window
    let mut window = Window::new("main");
    window.set_light(Light::StickToCamera);

    // Represent the point as a small sphere
    //let mut cone = window.add_cone(0.025, 0.05);
    //cone.set_local_translation(Translation3::new(p.x, p.y, p.z));

    /*
    let mut cow = window.add_obj(
        Path::new("cow.obj"),
        Path::new("./"),              // material directory ("" if none)
        Vector3::new(1.0, 1.0, 1.0), // scale
    );
    */

    let mut cone = window.add_cone(1.0, 2.0);

    let t = thruster_positions[0].translation.vector;
    cone.set_local_translation(Translation3::new(t.x, t.y, t.z));

    let rot: UnitQuaternion<f32> = thruster_positions[0].rotation; 
    cone.set_local_rotation(rot);

    odometry.apply_linear_acceleration(Vector3::new(0.0, 0.0, 0.0), 1.0);

    odometry.update_angular_velocity(Vector3::new(0.0, 0.0, 10.0));

    let size = 10;
    let step = 10.0;
    let half = size as f32 * step / 2.0;
    let grid_color = Point3::new(0.3, 0.3, 0.3);

    while window.render().await {
        for i in 0..=size {
            let p = -half + i as f32 * step;

            window.draw_line(
                &Point3::new(-half, 0.0, p),
                &Point3::new( half, 0.0, p),
                &grid_color,
            );

            window.draw_line(
                &Point3::new(p, 0.0, -half),
                &Point3::new(p, 0.0,  half),
                &grid_color,
            );
        }

        for evt in window.events().iter() {
            match evt.value {
                WindowEvent::Key(key, action, _) => match (key, action) {
                    (Key::W, Action::Press) => odometry.apply_linear_acceleration(Vector3::new(10.0, 0.0, 0.0), 0.01),
                    (Key::S, Action::Press) => odometry.apply_linear_acceleration(Vector3::new(-10.0, 0.0, 0.0), 0.01),
                    (Key::A, Action::Press) => odometry.apply_linear_acceleration(Vector3::new(0.0, 0.0, -10.0), 0.01),
                    (Key::D, Action::Press) => odometry.apply_linear_acceleration(Vector3::new(0.0, 0.0, 10.0), 0.01),
                    (Key::E, Action::Press) => publisher.publish("test", 100),
                    (Key::Escape, Action::Press) => return,
                    _ => {}
                },
                _ => {}
            }
        }


        match telemetry.receiver().recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                println!("Received telemetry event:");
                println!("Path: {}", event.path);
                println!("Value: {:?}", event.value);
                println!("Timestamp: {}", event.timestamp_ms);
            }
            Err(RecvTimeoutError::Timeout) => {
                println!("No telemetry event received in time");
            }
            Err(e) => {
                println!("Error receiving telemetry event: {:?}", e);
            }
        }

        /*
        // 1. Integrate odometry
        odometry.integrate(0.01);

        // 2. Get current pose
        let pose = odometry.pose();

        // 3. Update translation
        let t = pose.translation.vector; // Vector3<f32>
        cow.set_local_translation(Translation3::new(t.x, t.y, t.z));

        // 4. Update rotation
        let rot: UnitQuaternion<f32> = pose.rotation; 
        cow.set_local_rotation(rot);
        */

        sleep(time::Duration::from_millis(10));
    }
}
