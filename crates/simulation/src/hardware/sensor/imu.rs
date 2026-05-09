use nalgebra::Vector3;

use framework::hardware::interface::{Accelerometer, Gyroscope, Sensor};
use framework::physics::constants::STANDARD_GRAVITY;

use crate::data::context::SimContext;

pub struct SimImu {
	context: SimContext,
}

impl SimImu {
	pub fn new(context: SimContext) -> Self {
		Self { context }
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
		let condition = self.context.condition.read().unwrap();
		let gravity_world = Vector3::new(0.0, 0.0, -STANDARD_GRAVITY);
		let specific_force_world = condition.state.body.acceleration - gravity_world;
		let specific_force_body = condition.state.body.pose.rotation.inverse() * specific_force_world;
		let v = (specific_force_body / STANDARD_GRAVITY).cast::<f32>();
		Ok(Vector3::new(v.y, v.x, -v.z))
	}
}

impl Gyroscope<Vector3<f32>> for SimImu {
	fn read_rotation(&mut self) -> Result<Vector3<f32>, Self::Error> {
		let condition = self.context.condition.read().unwrap();
		let v = condition.state.body.twist.angular.cast::<f32>();
		Ok(Vector3::new(v.y, v.x, -v.z))
	}
}
