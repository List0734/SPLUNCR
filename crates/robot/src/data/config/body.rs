use serde::{Deserialize, Serialize};

use crate::platform::F;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BodyConfig {
	pub mass_properties: MassPropertiesConfig,
	pub buoyancy: BuoyancyConfig,
	pub drag: DragConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MassPropertiesConfig {
	pub mass: F,
	pub inertia: [F; 3],
	pub center: [F; 3],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuoyancyConfig {
	pub force: F,
	pub center: [F; 3],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DragConfig {
	pub linear: [F; 3],
	pub angular: [F; 3],
}
