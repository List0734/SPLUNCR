use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use crate::data::config::StationCommunicationConfig;
use crate::subsystem::communication::Communication;

use super::context::StationContext;

pub struct CommunicationService {
	context: StationContext,
	communication: Communication,
	poll_period: Duration,
	send_period: Duration,
	reconnect_interval: Duration,
}

impl CommunicationService {
	pub fn new(context: StationContext, communication: Communication, config: &StationCommunicationConfig) -> Self {
		Self {
			context,
			communication,
			poll_period: Duration::from_secs_f64(1.0 / config.poll_rate_hz as f64),
			send_period: Duration::from_secs_f64(1.0 / config.command.send_rate_hz as f64),
			reconnect_interval: Duration::from_millis(config.command.reconnect_interval_ms as u64),
		}
	}

	pub fn run(mut self) {
		let mut last_send = Instant::now();
		let mut last_connect_attempt = Instant::now() - self.reconnect_interval;
		let mut buf = [0u8; 65536];

		while !self.context.shutdown.load(Ordering::Relaxed) {
			if let Some(state) = self.communication.receive_telemetry(&mut buf) {
				self.context.condition.lock().unwrap().state = state;
			}

			if self.communication.is_connected() {
				if last_send.elapsed() >= self.send_period {
					let command = self.context.command.read().unwrap().clone();
					if self.communication.send_command(&command).is_err() {
						eprintln!("Command send failed, reconnecting");
						self.context.state.write().unwrap().communication.connected = false;
					}
					last_send = Instant::now();
				}
			} else if last_connect_attempt.elapsed() >= self.reconnect_interval {
				if self.communication.try_connect() {
					self.context.state.write().unwrap().communication.connected = true;
				}
				last_connect_attempt = Instant::now();
			}

			thread::sleep(self.poll_period);
		}
	}
}
