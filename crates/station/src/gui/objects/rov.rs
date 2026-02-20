use std::{fs::File, io::BufReader, path::Path};

use kiss3d::{parry3d::shape::TriMesh, scene::SceneNode, window::Window};
use nalgebra::{Isometry3, Quaternion, Translation3, Unit, UnitQuaternion, Vector3};
use robot::{data::condition::RobotCondition, platform::{F, subsystem::propulsion::NUM_THRUSTERS}};
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
        let mut cube = window.add_cube(2.0, 1.0, 1.2);
        //let mut cube = window.add_obj(Path::new("./test.obj"), Path::new(""), Vector3::new(0.5, 0.5, 0.5));
        cube.set_color(0.1, 0.5, 0.8);
        self.body_node = Some(cube);

        // Create thruster nodes
        for _ in 0..NUM_THRUSTERS {
            let mut thruster = window.add_cone(0.25, 0.5);
            thruster.set_color(1.0, 1.0, 1.0);
            self.thrust_nodes.push(thruster);
        }  
    }

    pub fn update(&mut self, robot: &RobotCondition) {
        // Update thruster positions
        let thrusters = robot.config.subsystem.propulsion.thrusters;
        for (thruster, node) in thrusters.iter().zip(self.thrust_nodes.iter_mut()) {
            let placement = thruster.placement;
            let position = Vector3::from(placement.position);
            let direction = Vector3::from(placement.direction);

            // Pre-rotation needed to align the cone +y up axis to +z
            let align_y_to_z = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::FRAC_PI_2);

            let target_rot = UnitQuaternion::rotation_between(&Vector3::z_axis(), &Unit::new_normalize(direction))
                .unwrap_or_default();

            // Combine the rotations: first align cone, then point in direction
            let rotation = target_rot * align_y_to_z;

            let iso = Isometry3::from_parts(
                Translation3::from(position),
                rotation
            );

            node.set_local_transformation(iso);
        }

        if let Some(node) = &mut self.body_node {
            let pose: Pose<F> = robot.state.estimator.odometry.pose.cast();
            node.set_local_transformation(pose);
        }
    }
}