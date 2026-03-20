pub mod communication;
pub mod context;
pub mod controller;
pub mod video;

use std::sync::atomic::Ordering;
use std::thread::{self, JoinHandle};

use context::StationContext;

pub struct Services {
	context: StationContext,
	handles: Vec<JoinHandle<()>>,
}

impl Services {
	pub fn launch(context: StationContext, tasks: Vec<Box<dyn FnOnce() + Send>>) -> Self {
		let handles = tasks
			.into_iter()
			.map(|task| thread::spawn(task))
			.collect();

		Self { context, handles }
	}

	pub fn shutdown(self) {
		self.context.shutdown.store(true, Ordering::Relaxed);
		for handle in self.handles {
			let _ = handle.join();
		}
	}
}
