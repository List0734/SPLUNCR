pub struct CommunicationState {
	pub connected: bool,
}

impl Default for CommunicationState {
	fn default() -> Self {
		Self { connected: false }
	}
}