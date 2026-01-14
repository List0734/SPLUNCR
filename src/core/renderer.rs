use kiss3d::{camera::ArcBall, window::Window};
use kiss3d::scene::SceneNode;
use kiss3d::light::Light;
use na::{Isometry3, UnitQuaternion, Vector3};
use nalgebra::{Point3, Translation3};

pub trait Renderable {
    fn render(&mut self, window: &mut Window);
}

pub struct Renderer {
    window: Window,
    objects: Vec<SceneNode>,
}

impl Renderer {
    pub fn new(title: &str) -> Self {
        let mut window = Window::new(title);
        window.set_light(Light::StickToCamera);

        Self {
            window,
            objects: Vec::new(),
        }
    }
}

pub trait WindowExt {
    fn add_vector(&mut self, iso: Isometry3<f32>, magnitude: f32, scale: f32) -> SceneNode;
    fn add_grid(&mut self, size: f32, spacing: f32, line_radius: f32) -> SceneNode;
}

impl WindowExt for Window {
    fn add_vector(&mut self, iso: Isometry3<f32>, magnitude: f32, scale: f32) -> SceneNode {
        // Arrow proportions
        let shaft_length = 0.6 * scale * magnitude;
        let head_length = 0.4 * scale * magnitude;
        let shaft_radius = 0.07 * scale;
        let head_radius = 0.25 * scale;

        let mut parent = self.add_group();

        let mut shaft = parent.add_cylinder(shaft_radius, shaft_length);
        shaft.append_translation(&Translation3::new(0.0, shaft_length / 2.0, 0.0));

        let mut head = parent.add_cone(head_radius, head_length);
        head.append_translation(&Translation3::new(0.0, shaft_length + head_length / 2.0, 0.0));

        parent.set_local_translation(iso.translation);
        parent.set_local_rotation(iso.rotation);

        parent
    }

    fn add_grid(&mut self, size: f32, spacing: f32, line_radius: f32) -> SceneNode {
        let mut grid_group = self.add_group();
        let count = (size / spacing).ceil() as i32;

        for i in -count..=count {
            let offset = i as f32 * spacing;

            // Horizontal line along X (at Y = offset)
            let mut line_x = grid_group.add_cylinder(line_radius, size * 2.0);
            // Rotate +Y -> +X
            line_x.append_rotation(&UnitQuaternion::from_euler_angles(0.0, 0.0, -std::f32::consts::FRAC_PI_2));
            line_x.append_translation(&Translation3::new(0.0, offset, 0.0));

            // Horizontal line along Y (at X = offset)
            let mut line_y = grid_group.add_cylinder(line_radius, size * 2.0);
            // +Y -> +Y (no rotation needed)
            line_y.append_translation(&Translation3::new(offset, 0.0, 0.0));
        }

        grid_group
    }
}