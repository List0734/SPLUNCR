use std::{fs::File, io::BufReader, path::Path};

use kiss3d::{parry3d::shape::TriMesh, scene::SceneNode, window::Window};
use nalgebra::{Isometry3, Translation3, Vector3};
use robot::{data::condition::RobotCondition, platform::F};
use shared::physics::kinematics::Pose;

pub struct RovObject {
    body_node: Option<SceneNode>,
    thrust_nodes: Vec<SceneNode>,
}

impl RovObject {
    pub fn new() -> Self {
        Self {
            body_node: None,
            thrust_nodes: Vec::new(),
        }
    }

    pub fn init(&mut self, window: &mut Window) {
        if self.body_node.is_none() {
            let mut cube = window.add_cube(2.0, 1.0, 1.2);
            //let mut cube = window.add_obj(Path::new("./test.obj"), Path::new(""), Vector3::new(0.5, 0.5, 0.5));
            cube.set_color(0.1, 0.5, 0.8);
            self.body_node = Some(cube);
        }
    }

    pub fn update(&mut self, model: &RobotCondition) {
        if let Some(node) = &mut self.body_node {
            let pose: Pose<F> = model.state.estimator.odometry.pose.cast();
            node.set_local_transformation(pose);
        }
    }
}