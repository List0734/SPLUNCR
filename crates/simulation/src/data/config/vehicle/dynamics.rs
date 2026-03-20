use serde::{Deserialize, Serialize};
use robot::platform::F;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DynamicsConfig {
	pub mass: F,
	pub drag: F,
}
