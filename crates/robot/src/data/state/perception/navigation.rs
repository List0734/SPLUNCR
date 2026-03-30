pub mod odometry;

use serde::{Serialize, Deserialize};

pub use odometry::OdometryState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NavigationPerception {
	pub odometry: OdometryState,
}

impl Default for NavigationPerception {
	fn default() -> Self {
		Self {
			odometry: OdometryState::default(),
		}
	}
}
