use std::sync::{Arc, Mutex};
use std::time::Instant;

use egui::Context;
use kiss3d::{camera::ArcBall, window::Window};
use nalgebra::{Point3, Vector3};
use robot::data::condition::RobotCondition;
use crate::data::video::VideoFrame;

use super::super::objects::{GridObject, RovObject};
use super::super::screens::{attitude_indicator, data_panel, TreeView, VideoOverlay};
use super::{Scene, SceneTransition};

pub struct StationaryScene {
    camera: ArcBall,
    rov: RovObject,
    grid: GridObject,
    video: VideoOverlay,
    last_frame: Instant,
    show_camera: bool,
    show_side_panel: bool,
    selected_tree: TreeView,
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
            video: VideoOverlay::new(),
            last_frame: Instant::now(),
            show_camera: true,
            show_side_panel: false,
            selected_tree: TreeView::State,
        }
    }

    fn draw_toolbar(&mut self, ctx: &Context) {
        let avail = ctx.available_rect();
        egui::Area::new(egui::Id::new("toolbar"))
            .fixed_pos(egui::pos2(avail.right() - 160.0, avail.top() + 10.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let cam_label = if self.show_camera { "Cam ON" } else { "Cam OFF" };
                        if ui.button(cam_label).clicked() {
                            self.show_camera = !self.show_camera;
                        }
                        let panel_label = if self.show_side_panel { "Close Panel" } else { "Open Panel" };
                        if ui.button(panel_label).clicked() {
                            self.show_side_panel = !self.show_side_panel;
                        }
                    });
                });
            });
    }

    fn draw_attitude_indicator(&self, ctx: &Context, robot: &RobotCondition) {
        let pose = &robot.state.perception.navigation.odometry.pose;
        let (roll, pitch, yaw) = pose.rotation.euler_angles();
        let depth = pose.translation.z;
        let heading = -yaw.to_degrees();

        let avail = ctx.available_rect();
        egui::Area::new(egui::Id::new("attitude_indicator"))
            .fixed_pos(egui::pos2(avail.right() - 230.0, avail.bottom() - 220.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                attitude_indicator(ui, 180.0, -roll.to_degrees(), pitch.to_degrees(), depth, heading);
            });
    }
}

impl Scene for StationaryScene {
    fn init(&mut self, window: &mut Window, robot: &RobotCondition) {
        self.rov.init(window, robot)
    }

    fn update_ui(&mut self, ctx: &Context, robot: &RobotCondition, video: &Arc<Mutex<Option<VideoFrame>>>) -> SceneTransition {
        self.video.process_frame(ctx, video);
        data_panel(ctx, &mut self.show_side_panel, &mut self.selected_tree, robot);
        self.draw_toolbar(ctx);
        self.video.draw(ctx, self.show_camera);
        self.draw_attitude_indicator(ctx, robot);
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
