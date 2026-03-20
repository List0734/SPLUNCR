pub mod odometry;

use serde::{Serialize, Deserialize};

pub use odometry::OdometryEstimatorState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NavigationPerception {
	pub odometry: OdometryEstimatorState,
}

impl Default for NavigationPerception {
	fn default() -> Self {
		Self {
			odometry: OdometryEstimatorState::default(),
		}
	}
}
