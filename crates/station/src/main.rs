use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use nalgebra::{Point3, UnitQuaternion};

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Async Kiss3d + Egui");
    
    let eye = Point3::new(3.0, 3.0, 3.0);
    let at = Point3::origin();
    let mut camera = ArcBall::new(eye, at);
    
    let mut cube = window.add_cube(1.0, 1.0, 1.0);
    cube.set_color(0.2, 0.7, 0.3);
    
    window.set_light(Light::StickToCamera);
    
    let mut rotation_x = 0.0_f32;
    let mut rotation_y = 0.0_f32;
    let mut rotation_z = 0.0_f32;
    let mut scale = 1.0_f32;
    
    while window.render_with_camera(&mut camera).await {
        // egui UI
        window.draw_ui(|ui| {
            egui::Window::new("Controls").show(ui, |ui| {
                ui.label("Rotation (radians):");
                ui.add(egui::Slider::new(&mut rotation_x, 0.0..=std::f32::consts::TAU).text("X"));
                ui.add(egui::Slider::new(&mut rotation_y, 0.0..=std::f32::consts::TAU).text("Y"));
                ui.add(egui::Slider::new(&mut rotation_z, 0.0..=std::f32::consts::TAU).text("Z"));
                ui.add(egui::Slider::new(&mut scale, 0.1..=3.0).text("Scale"));
            });
        });
        
        let rot = UnitQuaternion::from_euler_angles(rotation_x, rotation_y, rotation_z);
        cube.set_local_rotation(rot);
        cube.set_local_scale(scale, scale, scale);
    }
}