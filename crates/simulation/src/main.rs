use kiss3d;

use simulation::Simulation;

#[kiss3d::main]
async fn main() {
    println!("Simulation started");

    let mut simulation = Simulation::new();
    simulation.spawn_robot_thread();
    simulation.run_station_loop().await;
}