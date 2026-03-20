use std::sync::{Arc, Mutex};

use kiss3d::{camera::ArcBall, window::Window};
use egui::Context;

mod cube;
pub use cube::CubeScene;

mod stationary;
use crate::data::condition::RobotCondition;
pub use stationary::StationaryScene;

mod connecting;
pub use connecting::ConnectingScene;

use crate::data::video::VideoFrame;

pub enum SceneTransition {
    None,
    Switch(Box<dyn Scene>),
}

pub trait Scene {
    fn init(&mut self, window: &mut Window, robot: &RobotCondition);

    fn update_ui(&mut self, ctx: &Context, robot: &RobotCondition, video: &Arc<Mutex<Option<VideoFrame>>>) -> SceneTransition;

    fn update_3d(&mut self, window: &mut Window, robot: &RobotCondition) -> SceneTransition;

    fn camera(&mut self) -> &mut ArcBall;
}