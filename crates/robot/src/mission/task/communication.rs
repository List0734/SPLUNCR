use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use framework::hardware::interface::{Stream, Datagram};

use crate::data::command::OperatorCommand;
use crate::hardware::subsystem::CommunicationSubsystem;
use crate::mission::context::TaskContext;

pub struct CommunicationTask<C: Stream, T: Datagram> {
	context: TaskContext,
	communication: CommunicationSubsystem<C, T>,
	poll_period: Duration,
	telemetry_period: Duration,
}

impl<C: Stream, T: Datagram> CommunicationTask<C, T> {
	pub fn new(
		context: TaskContext,
		communication: CommunicationSubsystem<C, T>,
		poll_rate_hz: u32,
		telemetry_rate_hz: u32,
	) -> Self {
		Self {
			context,
			communication,
			poll_period: Duration::from_secs_f64(1.0 / poll_rate_hz as f64),
			telemetry_period: Duration::from_secs_f64(1.0 / telemetry_rate_hz as f64),
		}
	}

	pub fn run(mut self) {
		let mut command_buffer = [0u8; 4096];
		let mut last_telemetry = Instant::now();

		while !self.context.shutdown.load(Ordering::Relaxed) {
			self.receive_commands(&mut command_buffer);

			if last_telemetry.elapsed() >= self.telemetry_period {
				self.send_telemetry();
				last_telemetry = Instant::now();
			}

			thread::sleep(self.poll_period);
		}
	}

	fn receive_commands(&mut self, buffer: &mut [u8]) {
		while let Ok(Some(n)) = self.communication.command.try_receive(buffer) {
			let Ok(command) = bincode::deserialize::<OperatorCommand>(&buffer[..n]) else {
				eprintln!("ignored malformed command ({n} bytes)");
				continue;
			};
			*self.context.command.write().unwrap() = command;
		}
	}

	fn send_telemetry(&self) {
		let snapshot = self.context.state.read().unwrap().clone();
		if let Ok(bytes) = bincode::serialize(&snapshot) {
			let _ = self.communication.telemetry.send(&bytes);
		}
	}
}
