use framework::hardware::interface::{Stream, Datagram};

use crate::data::config::communication::CommunicationConfig;
use crate::data::config::Config;

pub struct CommunicationSubsystem<C: Stream, T: Datagram> {
	pub command: C,
	pub telemetry: T,
}

impl<C: Stream, T: Datagram> CommunicationSubsystem<C, T> {
	pub fn new(command: C, telemetry: T) -> Self {
		Self { command, telemetry }
	}
}

impl<C: Stream, T: Datagram> Config<CommunicationConfig> for CommunicationSubsystem<C, T> {
	fn update_config(&mut self, config: CommunicationConfig) {
		let _ = self.command.reconnect(&config.command.listen_address);
		let _ = self.telemetry.set_target(&config.telemetry.target_address);
	}
}
