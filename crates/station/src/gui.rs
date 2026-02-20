use std::sync::{Arc, Mutex};

use crossbeam::channel::Receiver;
use kiss3d::{camera::ArcBall, light::Light, window::Window};
use nalgebra::{Point3, Vector3};

pub mod scene;
use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use scene::Scene;
use shared::data::transport::message::Message;

use crate::{data::transport::telemetry::Mapper, gui::scene::SceneTransition};

pub mod objects;
pub mod screens;

pub struct Gui {
    window: Window,
    pub active_scene: Box<dyn Scene>,
}

impl Gui {
    pub fn new<S: Scene + 'static>(initial_scene: S) -> Self {
        let mut window = Window::new("Station");
        window.set_light(Light::StickToCamera);

        let mut gui = Self {
            window,
            active_scene: Box::new(initial_scene),
        };

        gui.active_scene.init(&mut gui.window);

        gui 
    }

    pub async fn run(&mut self, robot: Arc<Mutex<RobotCondition>>, telemetry: &Receiver<Message<State>>) {
        while self.window.render_with_camera(self.active_scene.camera()).await {
            // Pull messages from the receiver each frame
            while let Ok(msg) = telemetry.try_recv() {
                let mut robot_guard = robot.lock().unwrap();
                Mapper::ingest(&mut robot_guard, msg);
            }

            let robot_snapshot = { robot.lock().unwrap().clone() };

            // Draw UI
            let mut ui_transition = SceneTransition::None;
            self.window.draw_ui(|ctx| {
               ui_transition = self.active_scene.update_ui(ctx, &robot_snapshot);
            });

            // Update 3D
            let logic_transition = self.active_scene.update_3d(&mut self.window, &robot_snapshot);

            // Select scene transition (gives UI priority)
            let transition = match ui_transition {
                SceneTransition::None => logic_transition,
                other => other,
            };

            match transition {
                SceneTransition::Switch(mut new_scene) => {
                    new_scene.init(&mut self.window);
                    self.active_scene = new_scene;
                }
                SceneTransition::None => {}
            }
        }
    }

    pub fn switch_scene(&mut self, scene: Box<dyn Scene>) {
        self.active_scene = scene;
        self.active_scene.init(&mut self.window);
    }
}