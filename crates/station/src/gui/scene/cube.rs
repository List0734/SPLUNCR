use egui::Ui;
use kiss3d::{camera::ArcBall, scene::SceneNode, window::Window};
use nalgebra::{Point3, UnitQuaternion, Vector3};

pub struct CubeScene {
    camera: ArcBall,
    
    // Scene objects
    rotation_x: f32,
    rotation_y: f32,
    rotation_z: f32,
    scale: f32,
    cube: Option<SceneNode>,
}

impl CubeScene {
    pub fn new() -> Self {
        let eye = Point3::new(-20.0, -2.0, 8.0);
        let at  = Point3::origin();

        let mut camera = ArcBall::new(eye, at);
        camera.set_up_axis(Vector3::z());

        Self {
            camera,
            
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            scale: 1.0,
            cube: None,
        }
    }
}

impl CubeScene {
    pub fn init(&mut self, window: &mut Window) {
        if self.cube.is_none() {
            let mut cube = window.add_cube(1.0, 1.0, 1.0);
            cube.set_color(0.2, 0.7, 0.3);
            self.cube = Some(cube);
        }
    }

    pub fn update_ui(&mut self, ui: &mut Ui) {
        ui.label("Rotation (radians):");
        ui.add(egui::Slider::new(&mut self.rotation_x, 0.0..=std::f32::consts::TAU).text("X"));
        ui.add(egui::Slider::new(&mut self.rotation_y, 0.0..=std::f32::consts::TAU).text("Y"));
        ui.add(egui::Slider::new(&mut self.rotation_z, 0.0..=std::f32::consts::TAU).text("Z"));
        ui.add(egui::Slider::new(&mut self.scale, 0.1..=3.0).text("Scale"));
    }

    pub fn update_3d(&mut self, _window: &mut Window) {
        if let Some(cube) = &mut self.cube {
            let rot = UnitQuaternion::from_euler_angles(self.rotation_x, self.rotation_y, self.rotation_z);
            cube.set_local_rotation(rot);
            cube.set_local_scale(self.scale, self.scale, self.scale);
        }
    }

    pub fn camera(&mut self) -> &mut ArcBall {
        &mut self.camera
    }
}
