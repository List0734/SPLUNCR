use nalgebra::Vector3;

use framework::{hardware::interface::{Accelerometer, Gyroscope, Sensor}, physics::constants::STANDARD_GRAVITY};

pub struct SimImu;

impl SimImu {
	pub fn new() -> Self {
		Self
	}
}

impl Sensor for SimImu {
	type Error = std::convert::Infallible;

	fn calibrate(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl Accelerometer<Vector3<f32>> for SimImu {
	fn read_acceleration(&mut self) -> Result<Vector3<f32>, Self::Error> {
		Ok(Vector3::new(0.0, 0.0, -STANDARD_GRAVITY as f32))
	}
}

impl Gyroscope<Vector3<f32>> for SimImu {
	fn read_rotation(&mut self) -> Result<Vector3<f32>, Self::Error> {
		Ok(Vector3::zeros())
	}
}
