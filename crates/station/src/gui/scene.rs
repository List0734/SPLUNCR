use std::collections::HashMap;
use kiss3d::{camera::{ArcBall, Camera}, window::Window};
use nalgebra::UnitQuaternion;
use egui::Ui;

mod cube;
pub use cube::CubeScene;

mod stationary;
use robot::data::condition::RobotCondition;
pub use stationary::StationaryScene;

// Scenes
pub enum Scene {
    Cube(CubeScene),
    Stationary(StationaryScene),
}

impl Scene {
    pub fn camera(&mut self) -> &mut ArcBall {
        match self {
            Scene::Cube(s) => s.camera(),
            Scene::Stationary(s) => s.camera(),
        }
    }

    pub fn init(&mut self, window: &mut Window) {
        match self {
            Scene::Cube(s) => s.init(window),
            Scene::Stationary(s) => s.init(window),
        }
    }

    pub fn update_ui(&mut self, ui: &mut Ui, robot: &RobotCondition) {
        match self {
            Scene::Cube(s) => s.update_ui(ui),
            Scene::Stationary(s) => s.update_ui(ui, robot),
        }
    }

    pub fn update_3d(&mut self, window: &mut Window, robot: &RobotCondition) {
        match self {
            Scene::Cube(s) => s.update_3d(window),
            Scene::Stationary(s) => s.update_3d(window, robot),
        }
    }
}