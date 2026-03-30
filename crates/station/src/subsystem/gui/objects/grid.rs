use kiss3d::window::Window;
use nalgebra::{Point3, Vector3};

pub struct GridObject {
    spacing: f32,
    half_count: i32,
    extent: f32,
    fade_radius_sq: f32,
    offset: Vector3<f32>,
}

impl GridObject {
    pub fn new(spacing: f32, half_count: i32) -> Self {
        let extent = spacing * half_count as f32;
        Self {
            spacing,
            half_count,
            extent,
            fade_radius_sq: extent * extent * 0.5,
            offset: Vector3::zeros(),
        }
    }

    pub fn shift(&mut self, displacement: Vector3<f32>) {
        self.offset += displacement;
        for i in 0..3 {
            self.offset[i] = self.offset[i].rem_euclid(self.spacing);
        }
    }

    pub fn draw(&self, window: &mut Window) {
        let lo = -self.extent;
        let hi = self.extent;
        let o = self.offset;

        for i in -self.half_count..=self.half_count {
            for j in -self.half_count..=self.half_count {
                let a = i as f32 * self.spacing;
                let b = j as f32 * self.spacing;
                let dist_sq = a * a + b * b;
                if dist_sq > self.fade_radius_sq {
                    continue;
                }
                let t = 0.2 * (1.0 - dist_sq / self.fade_radius_sq);
                let color = Point3::new(t, t, t);

                window.draw_line(&Point3::new(lo, a + o.y, b + o.z), &Point3::new(hi, a + o.y, b + o.z), &color);
                window.draw_line(&Point3::new(a + o.x, lo, b + o.z), &Point3::new(a + o.x, hi, b + o.z), &color);
                window.draw_line(&Point3::new(a + o.x, b + o.y, lo), &Point3::new(a + o.x, b + o.y, hi), &color);
            }
        }
    }
}
