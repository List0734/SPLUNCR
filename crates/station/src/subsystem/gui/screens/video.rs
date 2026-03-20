use egui::{TextureHandle, Ui};

pub fn video_screen(ui: &mut Ui, texture: &TextureHandle, fps: f32, latency_ms: f32) {
    let size = texture.size_vec2();
    ui.label(format!("{}x{} | {:.1} FPS | {:.2} ms", size.x as u32, size.y as u32, fps, latency_ms));
    let available_width = ui.available_width();
    let scale = available_width / size.x;
    let display_size = egui::Vec2::new(size.x * scale, size.y * scale);
    ui.image(egui::load::SizedTexture::new(texture.id(), display_size));
}
