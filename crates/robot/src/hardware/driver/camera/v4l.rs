use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::capture::parameters::Parameters;
use v4l::video::Capture;
use v4l::{Format, FourCC};

use framework::hardware::interface::camera::{Camera, Frame};

pub struct V4lCamera {
	stream: MmapStream<'static>,
	width: u16,
	height: u16,
}

impl V4lCamera {
	pub fn new(device: &str, width: u16, height: u16, framerate: u32) -> Self {
		let dev = Device::with_path(device)
			.expect("failed to open camera device");

		let fmt = Format::new(width as u32, height as u32, FourCC::new(b"MJPG"));
		let actual = dev.set_format(&fmt).expect("failed to set camera format");

		dev.set_params(&Parameters::with_fps(framerate))
			.expect("failed to set camera framerate");

		if let Ok(params) = dev.params() {
			let interval = params.interval;
			println!("Camera: {}x{} {:?} @ {}/{} fps",
				actual.width, actual.height, actual.fourcc,
				interval.denominator, interval.numerator);
		} else {
			println!("Camera: {}x{} {:?}", actual.width, actual.height, actual.fourcc);
		}

		let stream = MmapStream::with_buffers(&dev, Type::VideoCapture, 4)
			.expect("failed to start camera stream");

		Self {
			stream,
			width: actual.width as u16,
			height: actual.height as u16,
		}
	}
}

impl Camera for V4lCamera {
	type Error = std::io::Error;

	fn capture(&mut self) -> Result<Frame<'_>, Self::Error> {
		let (buf, meta) = self.stream.next()?;
		Ok(Frame {
			data: &buf[..meta.bytesused as usize],
			width: self.width,
			height: self.height,
		})
	}
}
