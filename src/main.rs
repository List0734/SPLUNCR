extern crate kiss3d;
extern crate nalgebra as na;

mod system;
use system::System;

mod subsystem;

mod core;

#[cfg(feature = "simulation")]
mod simulation;

#[cfg(feature = "simulation")]
use simulation::Simulation;

#[cfg(feature = "simulation")]
#[kiss3d::main]
async fn main() {

    let system = System::new();

    let mut simulation = Simulation::new();

    println!("Simulation started");

    simulation.start(system).await;
}

#[cfg(not(feature = "simulation"))]
fn main() {

}
