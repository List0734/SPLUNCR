pub mod context;
pub mod task;

use std::sync::atomic::Ordering;
use std::thread::{self, JoinHandle};

use context::TaskContext;

pub struct Mission {
	context: TaskContext,
	handles: Vec<JoinHandle<()>>,
}

impl Mission {
	pub fn launch(context: TaskContext, tasks: Vec<Box<dyn FnOnce() + Send>>) -> Self {
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
