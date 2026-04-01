use std::sync::{Arc, Mutex};
use std::time::Instant;

use egui::{Align2, Color32, ColorImage, CornerRadius, Context, FontId, Sense, Stroke, StrokeKind, TextureHandle, TextureOptions, epaint::{Brush, RectShape}};
use crate::data::video::VideoFrame;

pub struct VideoOverlay {
    texture: Option<TextureHandle>,
    frame_count: u32,
    fps_timer: Instant,
    fps: f32,
    latency_ms: f32,
}

impl VideoOverlay {
    pub fn new() -> Self {
        Self {
            texture: None,
            frame_count: 0,
            fps_timer: Instant::now(),
            fps: 0.0,
            latency_ms: 0.0,
        }
    }

    pub fn process_frame(&mut self, ctx: &Context, video: &Arc<Mutex<Option<VideoFrame>>>) {
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

            match &mut self.texture {
                Some(tex) => tex.set(image, TextureOptions::LINEAR),
                None => {
                    self.texture = Some(ctx.load_texture("video_feed", image, TextureOptions::LINEAR));
                }
            }
        }
    }

    pub fn draw(&self, ctx: &Context, visible: bool) {
        if !visible {
            return;
        }

        egui::Area::new(egui::Id::new("video_overlay"))
            .fixed_pos(egui::pos2(10.0, 10.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let max_width = 320.0;
                match &self.texture {
                    Some(tex) => {
                        let size = tex.size_vec2();
                        let scale = max_width / size.x;
                        let display_size = egui::Vec2::new(max_width, size.y * scale);
                        let (rect, _) = ui.allocate_exact_size(display_size, Sense::hover());
                        ui.painter().add(RectShape {
                            rect,
                            corner_radius: CornerRadius::same(8),
                            fill: Color32::WHITE,
                            stroke: Stroke::NONE,
                            stroke_kind: StrokeKind::Inside,
                            round_to_pixels: None,
                            blur_width: 0.0,
                            brush: Some(Arc::new(Brush {
                                fill_texture_id: tex.id(),
                                uv: egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            })),
                        });
                        let stats = format!(
                            "{}x{} | {:.1} FPS | {:.1} ms",
                            size.x as u32, size.y as u32, self.fps, self.latency_ms,
                        );
                        ui.painter().text(
                            rect.left_bottom() + egui::vec2(4.0, -3.0),
                            Align2::LEFT_BOTTOM,
                            stats,
                            FontId::monospace(10.0),
                            Color32::WHITE,
                        );
                    }
                    None => {
                        let placeholder = egui::Vec2::new(max_width, max_width * 0.75);
                        let (rect, _) = ui.allocate_exact_size(placeholder, Sense::hover());
                        ui.painter().text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            "No Camera Feed",
                            FontId::proportional(16.0),
                            ui.visuals().text_color(),
                        );
                    }
                }
            });
    }
}
