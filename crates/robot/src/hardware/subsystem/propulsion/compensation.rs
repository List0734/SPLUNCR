use nalgebra::{UnitQuaternion, Vector3};
use framework::physics::constants::STANDARD_GRAVITY;
use framework::physics::dynamics::Wrench;

use crate::data::config::body::BodyConfig;
use crate::data::config::Config;
use crate::platform::F;

struct Weight {
	force: F,
	center: Vector3<F>,
}

struct Buoyancy {
	force: F,
	center: Vector3<F>,
}

#[allow(dead_code)]
struct Drag {
	linear: Vector3<F>,
	angular: Vector3<F>,
}

pub struct Compensation {
	weight: Weight,
	buoyancy: Buoyancy,
	#[allow(dead_code)]
	drag: Drag,
}

impl Compensation {
	pub fn new(config: &BodyConfig) -> Self {
		Self::from_config(config)
	}

	pub fn compute(&self, rotation: &UnitQuaternion<f64>) -> Wrench<F> {
		let inv_rotation = rotation.inverse();

		let net_vertical = Vector3::new(0.0, 0.0, (self.buoyancy.force - self.weight.force) as f64);
		let gravity_buoyancy_body: Vector3<F> = (inv_rotation * net_vertical).cast();

		let buoyancy_world = Vector3::new(0.0, 0.0, self.buoyancy.force as f64);
		let buoyancy_body: Vector3<F> = (inv_rotation * buoyancy_world).cast();
		let buoyancy_torque = (self.buoyancy.center - self.weight.center).cross(&buoyancy_body);

		Wrench {
			force: -gravity_buoyancy_body,
			torque: -buoyancy_torque,
		}
	}

	fn from_config(config: &BodyConfig) -> Self {
		Self {
			weight: Weight {
				force: config.mass_properties.mass * STANDARD_GRAVITY as F,
				center: Vector3::from(config.mass_properties.center),
			},
			buoyancy: Buoyancy {
				force: config.buoyancy.force,
				center: Vector3::from(config.buoyancy.center),
			},
			drag: Drag {
				linear: Vector3::from(config.drag.linear),
				angular: Vector3::from(config.drag.angular),
			},
		}
	}
}

impl Config<BodyConfig> for Compensation {
	fn update_config(&mut self, config: BodyConfig) {
		*self = Self::from_config(&config);
	}
}
