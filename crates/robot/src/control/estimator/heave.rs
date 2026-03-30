use crate::platform::Fp;

pub struct HeaveEstimator {
	prev_depth: Option<Fp>,
	velocity: Fp,
}

impl HeaveEstimator {
	pub fn new() -> Self {
		Self {
			prev_depth: None,
			velocity: 0.0,
		}
	}

	pub fn update(&mut self, depth: Fp, dt: Fp) {
		if let Some(prev) = self.prev_depth {
			self.velocity = (depth - prev) / dt;
		}
		self.prev_depth = Some(depth);
	}

	pub fn velocity(&self) -> Fp {
		self.velocity
	}

	pub fn depth(&self) -> Fp {
		self.prev_depth.unwrap_or(0.0)
	}
}