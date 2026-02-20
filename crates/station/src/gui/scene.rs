use std::collections::HashMap;
use kiss3d::{camera::{ArcBall, Camera}, window::Window};
use nalgebra::UnitQuaternion;
use egui::{Context, Ui};

mod cube;
pub use cube::CubeScene;

mod stationary;
use robot::data::condition::RobotCondition;
pub use stationary::StationaryScene;

mod connecting;
pub use connecting::ConnectingScene;

pub enum SceneTransition {
    None,
    Switch(Box<dyn Scene>),
}

pub trait Scene {
    fn init(&mut self, window: &mut Window);

    fn update_ui(&mut self, ctx: &Context, robot: &RobotCondition) -> SceneTransition;

    fn update_3d(&mut self, window: &mut Window, robot: &RobotCondition) -> SceneTransition;
    
    fn camera(&mut self) -> &mut ArcBall;
}