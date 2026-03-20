use std::sync::atomic::Ordering;

use framework::hardware::interface::{Camera, Datagram};

use crate::hardware::subsystem::VisionSubsystem;
use crate::mission::context::TaskContext;

pub struct VisionTask<C: Camera, D: Datagram> {
	context: TaskContext,
	vision: VisionSubsystem<C, D>,
}

impl<C: Camera, D: Datagram> VisionTask<C, D> {
	pub fn new(context: TaskContext, vision: VisionSubsystem<C, D>) -> Self {
		Self {
			context,
			vision,
		}
	}

	pub fn run(mut self) {
		while !self.context.shutdown.load(Ordering::Relaxed) {
			if let Err(error) = self.vision.capture_and_send() {
				eprintln!("vision error: {error}");
			}
		}
	}
}
