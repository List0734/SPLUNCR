#[cfg(feature = "camera")]
mod v4l;
#[cfg(feature = "camera")]
pub use v4l::V4lCamera;

#[cfg(feature = "picamera")]
mod picamera;
#[cfg(feature = "picamera")]
pub use picamera::PiCamera;
