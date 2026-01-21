use kiss3d::{scene::SceneNode, window::Window};
use nalgebra::Translation3;
use robot::data::condition::RobotCondition;

pub struct RovObject {
    body_node: Option<SceneNode>,
}

impl RovObject {
    pub fn new() -> Self {
        Self {
            body_node: None
        }
    }

    pub fn init(&mut self, window: &mut Window) {
        if self.body_node.is_none() {
            let mut cube = window.add_cube(2.0, 1.0, 1.2);
            cube.set_color(0.1, 0.5, 0.8);
            self.body_node = Some(cube);
        }
    }

    pub fn update(&mut self, model: &RobotCondition) {
        if let Some(node) = &mut self.body_node {
            let pose = model.state.odometry.pose;
            node.set_local_transformation(pose);
        }
    }
}