use egui::{RichText, Ui, text};
use kiss3d::{camera::ArcBall, scene::SceneNode, window::Window};
use nalgebra::{Point3, UnitQuaternion, Vector3};
use robot::data::condition::RobotCondition;

use crate::gui::objects::RovObject;

pub struct StationaryScene {
    camera: ArcBall,
    rov: RovObject,
}

impl StationaryScene {
    pub fn new() -> Self {
        let eye = Point3::new(-20.0, -2.0, 8.0);
        let at  = Point3::origin();

        let mut camera = ArcBall::new(eye, at);
        camera.set_up_axis(Vector3::z());

        Self {
            camera, 
            rov: RovObject::new(),
        }
    }
}

impl StationaryScene {
    pub fn init(&mut self, window: &mut Window) {
        self.rov.init(window)
    }

    pub fn update_ui(&mut self, ui: &mut Ui, robot: &RobotCondition) {
        let rotation: UnitQuaternion<f64> = robot.state.estimator.odometry.pose.rotation;

        let (roll, pitch, yaw) = rotation.euler_angles();

        ui.horizontal(|ui| {
            ui.label("Roll (rad):");
            ui.label(RichText::new(format!("{:.3}", roll)));
        });

        ui.horizontal(|ui| {
            ui.label("Pitch (rad):");
            ui.label(RichText::new(format!("{:.3}", pitch)));
        });

        ui.horizontal(|ui| {
            ui.label("Yaw (rad):");
            ui.label(RichText::new(format!("{:.3}", yaw)));
        });
    }

    pub fn update_3d(&mut self, _window: &mut Window, robot: &RobotCondition) {
        self.rov.update(robot)
    }

    pub fn camera(&mut self) -> &mut ArcBall {
        &mut self.camera
    }
}
