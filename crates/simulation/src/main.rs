use kiss3d;

use robot::Robot;

mod simulation;
use simulation::Simulation;

#[kiss3d::main]
async fn main() {
    println!("Simulation started");

    let system = Robot::new();

    let mut simulation = Simulation::new();

    simulation.start(system).await;
}