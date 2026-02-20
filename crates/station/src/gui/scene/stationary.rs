use egui::{Context, RichText, Ui, text};
use kiss3d::{camera::ArcBall, scene::SceneNode, window::Window};
use nalgebra::{Point3, UnitQuaternion, Vector3};
use robot::data::condition::RobotCondition;

use crate::gui::{objects::RovObject, scene::{Scene, SceneTransition}, screens::{config_screen, state_screen}};

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

impl Scene for StationaryScene {
    fn init(&mut self, window: &mut Window) {
        self.rov.init(window)
    }

    fn update_ui(&mut self, ctx: &Context, robot: &RobotCondition) -> SceneTransition {
        egui::Window::new("Robot State")
            .collapsible(true)
            .resizable(true)
            .max_size(ctx.available_rect().size())
            .show(ctx, |ui| {
                state_screen(ui, &robot.state);
        });

        egui::Window::new("Robot Config")
            .collapsible(true)
            .resizable(true)
            .max_size(ctx.available_rect().size())
            .show(ctx, |ui| {
                config_screen(ui, &robot.config);
        });

        SceneTransition::None
    }

    fn update_3d(&mut self, _window: &mut Window, robot: &RobotCondition) -> SceneTransition {
        self.rov.update(robot);
        SceneTransition::None
    }

    fn camera(&mut self) -> &mut ArcBall {
        &mut self.camera
    }
}
