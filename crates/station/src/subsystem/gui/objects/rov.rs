use kiss3d::{scene::SceneNode, window::Window};
use nalgebra::{Isometry3, Translation3, Unit, UnitQuaternion, Vector3};
use robot::platform::F;
use framework::physics::kinematics::Pose;

use crate::data::condition::RobotCondition;

const THRUSTER_CONE_SIZE: f32 = 0.5;

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

    pub fn init(&mut self, window: &mut Window, robot: &RobotCondition) {
        let mut root = window.add_group();

        let mut hull = root.add_cube(2.0, 1.0, 1.2);
        hull.set_color(0.1, 0.5, 0.8);

        for thruster in robot.config.propulsion.thrusters.iter() {
            let position = Vector3::from(thruster.placement.position);
            let direction = Vector3::from(thruster.placement.direction);

            let align_y_to_z = UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                std::f32::consts::FRAC_PI_2,
            );
            let target_rot = UnitQuaternion::rotation_between(
                &Vector3::z_axis(),
                &Unit::new_normalize(direction),
            )
            .unwrap_or_default();

            let mut group = root.add_group();
            group.set_local_transformation(Isometry3::from_parts(
                Translation3::from(position),
                target_rot * align_y_to_z,
            ));

            let mut cone = group.add_cone(THRUSTER_CONE_SIZE / 2.0, THRUSTER_CONE_SIZE);
            cone.set_color(1.0, 1.0, 1.0);
            self.thrust_nodes.push(cone);
        }

        self.body_node = Some(root);
    }

    pub fn update(&mut self, robot: &RobotCondition) {
        let output = &robot.state.action.propulsion.thruster.coast.output;
        for (node, &thrust) in self.thrust_nodes.iter_mut().zip(output.iter()) {
            let t = thrust.abs().clamp(0.0, 1.0);
            let s = THRUSTER_CONE_SIZE * t;
            node.set_local_scale(s, s, s);
            if thrust >= 0.0 {
                node.set_local_translation(Translation3::new(0.0, s / 2.0, 0.0));
                node.set_local_rotation(UnitQuaternion::identity());
            } else {
                node.set_local_translation(Translation3::new(0.0, -s / 2.0, 0.0));
                node.set_local_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    std::f32::consts::PI,
                ));
            }
        }

        if let Some(node) = &mut self.body_node {
            let pose: Pose<F> = robot.state.perception.navigation.odometry.pose.cast();
            node.set_local_transformation(pose);
        }
    }
}