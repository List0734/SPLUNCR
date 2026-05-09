use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraConfig {
	pub device: String,
	pub width: u16,
	pub height: u16,
	pub framerate: u32,
	#[serde(default)]
	pub flip_vertical: bool,
	#[serde(default)]
	pub flip_horizontal: bool,
}
