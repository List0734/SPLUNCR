pub mod propulsion;
pub use propulsion::Propulsion;

pub mod odometry;
pub use odometry::Odometry;

pub enum Subsystem {
    Propulsion(Propulsion),
    Odometry(Odometry),
}