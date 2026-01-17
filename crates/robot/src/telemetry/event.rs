pub mod subsystem;
pub use subsystem::Subsystem;

pub enum Event {
    Subsystem(Subsystem),
}