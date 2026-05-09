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
	flip_vertical: bool,
	flip_horizontal: bool,
	flip_buf: Vec<u8>,
}

impl V4lCamera {
	pub fn new(device: &str, width: u16, height: u16, framerate: u32, flip_vertical: bool, flip_horizontal: bool) -> Self {
		let dev = Device::with_path(device)
			.expect("failed to open camera device");

		let fmt = Format::new(width as u32, height as u32, FourCC::new(b"YUYV"));
		let actual = dev.set_format(&fmt).expect("failed to set camera format");

		let _ = dev.set_params(&Parameters::with_fps(framerate));

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

		let w = actual.width as u16;
		let h = actual.height as u16;
		let flip_buf = if flip_vertical || flip_horizontal {
			vec![0u8; w as usize * h as usize * 2]
		} else {
			Vec::new()
		};

		Self {
			stream,
			width: w,
			height: h,
			flip_vertical,
			flip_horizontal,
			flip_buf,
		}
	}
}

impl Camera for V4lCamera {
	type Error = std::io::Error;

	fn capture(&mut self) -> Result<Frame<'_>, Self::Error> {
		let (buf, meta) = self.stream.next()?;
		let data = &buf[..meta.bytesused as usize];

		if self.flip_vertical || self.flip_horizontal {
			let row_size = self.width as usize * 2;
			let macropixels = self.width as usize / 2;
			for row in 0..self.height as usize {
				let src_row = &data[row * row_size..(row + 1) * row_size];
				let dst_row_idx = if self.flip_vertical { self.height as usize - 1 - row } else { row };
				let dst_row = &mut self.flip_buf[dst_row_idx * row_size..(dst_row_idx + 1) * row_size];
				if self.flip_horizontal {
					for mp in 0..macropixels {
						let s = mp * 4;
						let d = (macropixels - 1 - mp) * 4;
						dst_row[d]     = src_row[s + 2];
						dst_row[d + 1] = src_row[s + 1];
						dst_row[d + 2] = src_row[s];
						dst_row[d + 3] = src_row[s + 3];
					}
				} else {
					dst_row.copy_from_slice(src_row);
				}
			}
			Ok(Frame {
				data: &self.flip_buf,
				width: self.width,
				height: self.height,
			})
		} else {
			Ok(Frame {
				data,
				width: self.width,
				height: self.height,
			})
		}
	}
}
