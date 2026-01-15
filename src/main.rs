extern crate kiss3d;
extern crate nalgebra as na;

#[cfg(feature = "simulation")]
mod simulation;

#[cfg(feature = "simulation")]
use simulation::Simulation;

#[cfg(feature = "simulation")]


#[cfg(not(feature = "simulation"))]
fn main() {

}
