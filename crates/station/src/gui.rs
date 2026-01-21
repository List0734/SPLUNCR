use std::sync::{Arc, Mutex};

use crossbeam::channel::Receiver;
use kiss3d::{camera::ArcBall, light::Light, window::Window};
use nalgebra::{Point3, Vector3};

pub mod scene;
use robot::data::{condition::RobotCondition, transport::telemetry::Message};
use scene::Scene;

use crate::data::transport::telemetry::Mapper;

pub mod objects;

pub struct Gui {
    window: Window,
    pub active_scene: Scene,
}

impl Gui {
    pub fn new(initial_scene: Scene) -> Self {
        let mut window = Window::new("Station");
        window.set_light(Light::StickToCamera);

        let mut gui = Self {
            window,
            active_scene: initial_scene,
        };

        gui.active_scene.init(&mut gui.window);

        gui 
    }

    pub async fn run(&mut self, robot: Arc<Mutex<RobotCondition>>, telemetry: &Receiver<Message>) {
        while self.window.render_with_camera(self.active_scene.camera()).await {
            // Pull messages from the receiver each frame
            while let Ok(msg) = telemetry.try_recv() {
                let mut robot_guard = robot.lock().unwrap();
                Mapper::ingest(&mut robot_guard, msg);
            }

            // Initialize scene if not already
            //self.active_scene.init(&mut self.window);

            let robot_snapshot = { robot.lock().unwrap().clone() };

            // Draw UI
            self.window.draw_ui(|ctx| {
                egui::Window::new("Controls").show(ctx, |ui| {
                    self.active_scene.update_ui(ui, &robot_snapshot);
                });
            });

            // Render 3D
            self.active_scene.update_3d(&mut self.window, &robot_snapshot);
        }
    }

    pub fn switch_scene(&mut self, scene: Scene) {
        self.active_scene = scene;
        self.active_scene.init(&mut self.window);
    }
}