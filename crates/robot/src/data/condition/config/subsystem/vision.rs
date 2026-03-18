use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VisionConfig {
    pub camera: CameraConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CameraConfig {
    pub device: String,
    pub width: u32,
    pub height: u32,
}
