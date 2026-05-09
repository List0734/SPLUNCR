use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use kiss3d::{
    resource::{GpuMesh, vertex_index::VertexIndex},
    scene::SceneNode,
    window::Window,
};
use nalgebra::{Isometry3, Point3, Translation3, Unit, UnitQuaternion, Vector3};
use robot::platform::F;
use framework::physics::kinematics::Pose;

use robot::data::condition::RobotCondition;

const THRUSTER_CONE_SIZE: f32 = 0.06;
const THRUSTER_CONE_OFFSET: f32 = 0.05;
const HULL_OBJ_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/SPLUNCR.obj");

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

        let mut hull = root.add_mesh(load_hull_mesh(Path::new(HULL_OBJ_PATH)), Vector3::new(1.0, 1.0, 1.0));
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
        let thrusters = &robot.config.propulsion.thrusters;
        for ((node, &force), thruster) in self.thrust_nodes.iter_mut().zip(output.iter()).zip(thrusters.iter()) {
            let max = thruster.max_force.map(|max_force| if force >= 0.0 { max_force.forward } else { max_force.reverse }).unwrap_or(1.0);
            let t = (force.abs() / max).clamp(0.0, 1.0);
            let s = THRUSTER_CONE_SIZE * t;
            node.set_local_scale(s, s, s);
            if force >= 0.0 {
                node.set_local_translation(Translation3::new(0.0, THRUSTER_CONE_OFFSET + s / 2.0, 0.0));
                node.set_local_rotation(UnitQuaternion::identity());
            } else {
                node.set_local_translation(Translation3::new(0.0, -THRUSTER_CONE_OFFSET - s / 2.0, 0.0));
                node.set_local_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    std::f32::consts::PI,
                ));
            }
        }

        if let Some(node) = &mut self.body_node {
            let pose: Pose<F> = robot.state.perception.navigation.odometry.pose.cast();
            node.set_local_transformation(Isometry3::from_parts(
                Translation3::identity(),
                pose.rotation,
            ));
        }
    }
}

fn load_hull_mesh(path: &Path) -> Rc<RefCell<GpuMesh>> {
    let text = std::fs::read_to_string(path).expect("failed to read hull obj");
    let mut coords: Vec<Point3<f32>> = Vec::new();
    let mut faces: Vec<Point3<VertexIndex>> = Vec::new();

    for line in text.lines() {
        let mut parts = line.split_ascii_whitespace();
        match parts.next() {
            Some("v") => {
                let x = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let y = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let z = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                coords.push(Point3::new(x, y, z));
            }
            Some("f") => {
                let n = coords.len() as i64;
                let idx: Vec<VertexIndex> = parts
                    .filter_map(|w| {
                        let v: i64 = w.split('/').next()?.parse().ok()?;
                        let abs = if v > 0 { v - 1 } else { n + v };
                        (abs >= 0 && abs < n).then_some(abs as VertexIndex)
                    })
                    .collect();
                for i in 1..idx.len().saturating_sub(1) {
                    faces.push(Point3::new(idx[0], idx[i], idx[i + 1]));
                }
            }
            _ => {}
        }
    }

    Rc::new(RefCell::new(GpuMesh::new(coords, faces, None, None, false)))
}