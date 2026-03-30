use serde::{Deserialize, Serialize};

use framework::control::filters::MahonyConfig;

use crate::platform::{F, Fp};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct AttitudeEstimatorConfig {
	#[serde(flatten)]
	pub mahony: MahonyConfig<Fp>,
	pub acceleration_tolerance: F,
}
