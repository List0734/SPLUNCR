pub mod camera;
pub mod motor;
pub mod sensor;
pub mod socket;

pub use camera::{Camera, Frame};
pub use motor::Motor;
pub use sensor::{Sensor, Thermometer, Barometer, Accelerometer, Gyroscope};
pub use socket::{Stream, Datagram};
