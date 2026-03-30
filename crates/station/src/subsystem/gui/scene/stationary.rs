use std::sync::{Arc, Mutex};
use std::time::Instant;

use egui::{Context, ColorImage, TextureHandle, TextureOptions};
use kiss3d::{camera::ArcBall, window::Window};
use nalgebra::{Point3, Vector3};
use robot::data::condition::RobotCondition;
use crate::data::video::VideoFrame;

use super::super::objects::{GridObject, RovObject};
use super::super::screens::{config_screen, state_screen, video_screen};
use super::{Scene, SceneTransition};

pub struct StationaryScene {
    camera: ArcBall,
    rov: RovObject,
    grid: GridObject,
    video_texture: Option<TextureHandle>,
    frame_count: u32,
    fps_timer: Instant,
    fps: f32,
    latency_ms: f32,
    last_frame: Instant,
}

impl StationaryScene {
    pub fn new() -> Self {
        let eye = Point3::new(-20.0, -2.0, 8.0);
        let at  = Point3::origin();

        let mut camera = ArcBall::new(eye, at);
        camera.set_up_axis(Vector3::z());
        camera.set_dist_step(1.0 / 1.01);
        camera.rebind_drag_button(None);

        Self {
            camera,
            rov: RovObject::new(),
            grid: GridObject::new(4.0, 6),
            video_texture: None,
            frame_count: 0,
            fps_timer: Instant::now(),
            fps: 0.0,
            latency_ms: 0.0,
            last_frame: Instant::now(),
        }
    }
}

impl Scene for StationaryScene {
    fn init(&mut self, window: &mut Window, robot: &RobotCondition) {
        self.rov.init(window, robot)
    }

    fn update_ui(&mut self, ctx: &Context, robot: &RobotCondition, video: &Arc<Mutex<Option<VideoFrame>>>) -> SceneTransition {
        if let Some(frame) = video.lock().unwrap().take() {
            self.frame_count += 1;
            let elapsed = self.fps_timer.elapsed().as_secs_f32();
            if elapsed >= 1.0 {
                self.fps = self.frame_count as f32 / elapsed;
                self.frame_count = 0;
                self.fps_timer = Instant::now();
            }

            self.latency_ms = frame.latency_ms;

            let image = ColorImage::from_rgba_unmultiplied(
                [frame.width as usize, frame.height as usize],
                &frame.pixels,
            );

            match &mut self.video_texture {
                Some(tex) => tex.set(image, TextureOptions::LINEAR),
                None => {
                    self.video_texture = Some(ctx.load_texture("video_feed", image, TextureOptions::LINEAR));
                }
            }
        }

        if let Some(tex) = &self.video_texture {
            egui::Window::new("Camera Feed")
                .collapsible(true)
                .resizable(true)
                .max_size(ctx.available_rect().size())
                .show(ctx, |ui| {
                    video_screen(ui, tex, self.fps, self.latency_ms);
                });
        }

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

    fn update_3d(&mut self, window: &mut Window, robot: &RobotCondition) -> SceneTransition {
        self.rov.update(robot);
        self.camera.set_at(Point3::origin());

        let dt = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = Instant::now();
        let velocity: Vector3<f32> = robot.state.perception.navigation.odometry.twist.linear.cast();
        self.grid.shift(-velocity * dt);
        self.grid.draw(window);

        SceneTransition::None
    }

    fn camera(&mut self) -> &mut ArcBall {
        &mut self.camera
    }
}
