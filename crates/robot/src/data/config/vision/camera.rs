use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraConfig {
	pub device: String,
	pub width: u16,
	pub height: u16,
	pub framerate: u32,
}
