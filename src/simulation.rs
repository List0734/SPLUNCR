use kiss3d::{camera::ArcBall, light::Light, window::Window};
use na::{Isometry3, Point3, UnitQuaternion, Vector3};
use std::{path::Path, thread};

use crate::{core::renderer::WindowExt, subsystem::Odometry, system::System};

pub struct Simulation {
    window: Window,
}

impl Simulation {
    pub fn new() -> Self {
       let mut window = Window::new("simulation");
        window.set_light(Light::StickToCamera);

        Self {
            window
        }
    }

    pub async fn start(&mut self, system: System) {
        let mut thruster_arrows: Vec<kiss3d::scene::SceneNode> = Vec::new();

        for pose in system.propulsion.lock().unwrap().thruster_positions() {
            let arrow = self.window.add_vector(pose, 1.0, 2.0);
            thruster_arrows.push(arrow);
        }

        let mut odometry_pose_arrow = self.window.add_vector(*system.odometry.lock().unwrap().pose(), 1.0, 4.0);

        let mut mesh = self.window.add_obj(Path::new("test.obj"), Path::new(""), Vector3::new(0.5, 0.5, 0.5));

        self.window.add_grid(10.0, 1.0, 0.005);
        
        let eye = Point3::new(-20.0, -2.0, 8.0);
        let at  = Point3::origin();

        let mut camera = ArcBall::new(eye, at);
        camera.set_up_axis(Vector3::z());

        system.odometry.lock().unwrap().apply_linear_acceleration(Vector3::new(2.0, 2.0, 0.0), 0.1);
        system.odometry.lock().unwrap().update_angular_velocity(Vector3::new(0.5, 0.5, 0.0));

        system.propulsion.lock().unwrap().test_telemetry();

        let rx = system.telemetry.receiver().clone();

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        println!("[Telemetry] {:?}", event);
                    }
                    Err(_) => {
                        // Sender has been dropped, exit the loop
                        break;
                    }
                }

                // Optional: sleep a tiny bit to reduce busy-waiting
                // thread::sleep(Duration::from_millis(1));
            }
        });

        while self.window.render_with_camera(&mut camera).await {

            let odometry_pose = system.odometry.lock().unwrap().pose().clone();

            // Update Thruster Arrows
            let poses = system.propulsion.lock().unwrap().thruster_positions();

            for (arrow, pose) in thruster_arrows.iter_mut().zip(poses.iter()) {
                let global_pose = odometry_pose * pose;
                arrow.set_local_translation(global_pose.translation);
                arrow.set_local_rotation(global_pose.rotation);
            }

            // Update Odometry Arrow
            odometry_pose_arrow.set_local_transformation(odometry_pose);

            // Update Mesh Position
            let rot = UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_2, 0.0, std::f32::consts::FRAC_PI_2);
            let new_pose = odometry_pose * Isometry3::from_parts(Vector3::zeros().into(), rot);
            mesh.set_local_transformation(new_pose);

            system.odometry.lock().unwrap().integrate(0.01);

        }
    }
}