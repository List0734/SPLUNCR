use std::sync::{Arc, Mutex, RwLock};

use egui::Context;
use kiss3d::{camera::ArcBall, window::Window};
use nalgebra::{Point3, Vector3};

use crate::data::condition::RobotCondition;
use crate::data::state::StationState;
use crate::data::video::VideoFrame;

use super::{StationaryScene, Scene, SceneTransition};

pub struct ConnectingScene {
    camera: ArcBall,
    state: Arc<RwLock<StationState>>,
}

impl ConnectingScene {
    pub fn new(state: Arc<RwLock<StationState>>) -> Self {
        let eye = Point3::new(0.0, -10.0, 5.0);
        let at  = Point3::origin();

        let mut camera = ArcBall::new(eye, at);
        camera.set_up_axis(Vector3::z());

        Self { camera, state }
    }
}

impl Scene for ConnectingScene {
    fn init(&mut self, _window: &mut Window, _robot: &RobotCondition) {}

    fn update_ui(&mut self, ctx: &Context, _robot: &RobotCondition, _video: &Arc<Mutex<Option<VideoFrame>>>) -> SceneTransition {
        if self.state.read().unwrap().communication.connected {
            return SceneTransition::Switch(Box::new(StationaryScene::new()));
        }

        let mut transition = SceneTransition::None;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {

                let available_height = ui.available_height();
                let vertical_offset = available_height / 2.0 - 40.0;
                ui.add_space(vertical_offset);

                ui.vertical_centered(|ui| {
                    ui.heading("Connecting to Robot...");
                    ui.add_space(10.0);
                    ui.spinner();
                    ui.add_space(10.0);

                    if ui.button("Skip").clicked() {
                        transition = SceneTransition::Switch(
                            Box::new(StationaryScene::new())
                        );
                    }
                });
            });

        transition
    }

    fn update_3d(&mut self, _window: &mut Window, _robot: &RobotCondition) -> SceneTransition {
        SceneTransition::None
    }

    fn camera(&mut self) -> &mut ArcBall {
        &mut self.camera
    }
}
