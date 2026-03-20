mod communication;
pub use communication::CommunicationState;

pub struct StationState {
	pub communication: CommunicationState,
}

impl Default for StationState {
	fn default() -> Self {
		Self {
			communication: CommunicationState::default(),
		}
	}
}
