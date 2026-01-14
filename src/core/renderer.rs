use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::light::Light;
use nalgebra::{Point3, Translation3};

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

    // Persistent objects

    pub fn add_point(&mut self, position: Point3<f32>, radius: f32) -> usize {
        let mut node = self.window.add_sphere(radius);
        node.set_local_translation(Translation3::new(
            position.x,
            position.y,
            position.z,
        ));

        self.objects.push(node);
        self.objects.len() - 1
    }

    pub fn add_cube(&mut self, size: f32, position: Point3<f32>) -> usize {
        let mut node = self.window.add_cube(size, size, size);
        node.set_local_translation(Translation3::new(
            position.x,
            position.y,
            position.z,
        ));

        self.objects.push(node);
        self.objects.len() - 1
    }

    pub fn update_position(&mut self, index: usize, position: Point3<f32>) {
        if let Some(node) = self.objects.get_mut(index) {
            node.set_local_translation(Translation3::new(
                position.x,
                position.y,
                position.z,
            ));
        }
    }

    // Stateless drawing

    pub fn draw_grid(&mut self, lines: i32, spacing: f32) {
        let half = lines as f32 * spacing / 2.0;
        let color = Point3::new(0.6, 0.6, 0.6);

        for i in 0..=lines {
            let p = i as f32 * spacing - half;

            self.window.draw_line(
                &Point3::new(p, 0.0, -half),
                &Point3::new(p, 0.0, half),
                &color,
            );

            self.window.draw_line(
                &Point3::new(-half, 0.0, p),
                &Point3::new(half, 0.0, p),
                &color,
            );
        }
    }

    pub fn draw_axes(&mut self, scale: f32) {
        self.window.draw_line(
            &Point3::origin(),
            &Point3::new(scale, 0.0, 0.0),
            &Point3::new(1.0, 0.0, 0.0),
        );
        self.window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, scale, 0.0),
            &Point3::new(0.0, 1.0, 0.0),
        );
        self.window.draw_line(
            &Point3::origin(),
            &Point3::new(0.0, 0.0, scale),
            &Point3::new(0.0, 0.0, 1.0),
        );
    }

    // Render loop

    pub async fn run(mut self) {
        while self.window.render().await {
            // draw_grid / debug visuals can be called here if dynamic
        }
    }
}