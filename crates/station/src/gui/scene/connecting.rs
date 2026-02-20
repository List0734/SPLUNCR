use egui::{Context, Ui};
use kiss3d::{camera::ArcBall, window::Window};
use nalgebra::{Point3, Vector3};
use robot::data::condition::RobotCondition;

use crate::gui::scene::{StationaryScene, Scene, SceneTransition};

pub struct ConnectingScene {
    camera: ArcBall,
}

impl ConnectingScene {
    pub fn new() -> Self {
        let eye = Point3::new(0.0, -10.0, 5.0);
        let at  = Point3::origin();

        let mut camera = ArcBall::new(eye, at);
        camera.set_up_axis(Vector3::z());

        Self { camera }
    }
}

impl Scene for ConnectingScene {
    fn init(&mut self, _window: &mut Window) {
        // No 3D objects required
    }

    fn update_ui(&mut self, ctx: &Context, _robot: &RobotCondition) -> SceneTransition {
        let mut transition = SceneTransition::None;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {

                let available_height = ui.available_height();
                let vertical_offset = available_height / 2.0 - 40.0; // adjust as needed
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