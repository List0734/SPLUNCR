use nalgebra::Vector3;

use framework::hardware::interface::{Sensor, Accelerometer, Gyroscope};

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
		Ok(Vector3::zeros())
	}
}

impl Gyroscope<Vector3<f32>> for SimImu {
	fn read_rotation(&mut self) -> Result<Vector3<f32>, Self::Error> {
		Ok(Vector3::zeros())
	}
}
