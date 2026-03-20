use serde::{Deserialize, Serialize};

mod camera;
pub use camera::CameraConfig;

mod stream;
pub use stream::VideoConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VisionConfig {
	pub camera: CameraConfig,
	pub stream: VideoConfig,
}
