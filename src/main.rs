extern crate kiss3d;
extern crate nalgebra as na;

mod system;
use system::System;

mod core;
use crate::core::simulation::Simulation;

mod subsystem;

#[kiss3d::main]
async fn main() {

    let system = System::new();

    let mut simulation = Simulation::new();

    println!("Program started");

    simulation.start(system).await;
}
