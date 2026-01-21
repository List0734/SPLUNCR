use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::camera::ArcBall;
use nalgebra::{Point3, UnitQuaternion};
use station::{Gui, gui::scene::{CubeScene, Scene}};

#[kiss3d::main]
async fn main() {
    let initial_scene = Scene::Cube(CubeScene::new());
    let mut gui = Gui::new(initial_scene);

    // Run GUI main loop (camera is now fully owned by Gui)
    //gui.run().await;
}