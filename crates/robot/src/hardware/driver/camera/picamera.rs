use std::io::Read;
use std::process::{Child, Command, Stdio};

use framework::hardware::interface::camera::{Camera, Frame};

const JPEG_SOI: [u8; 2] = [0xFF, 0xD8];
const JPEG_EOI: [u8; 2] = [0xFF, 0xD9];
const READ_CHUNK: usize = 65536;

pub struct PiCamera {
	child: Child,
	pending: Vec<u8>,
	frame_buf: Vec<u8>,
	width: u16,
	height: u16,
}

impl PiCamera {
	pub fn new(_device: &str, width: u16, height: u16, framerate: u32, flip_vertical: bool, flip_horizontal: bool) -> Self {
		let width_string = width.to_string();
		let height_string = height.to_string();
		let fps_string = framerate.to_string();
		let mut args = vec![
			"--codec", "mjpeg",
			"--width", &width_string,
			"--height", &height_string,
			"--framerate", &fps_string,
			"--timeout", "0",
			"--nopreview",
			"--output", "-",
		];
		if flip_vertical {
			args.push("--vflip");
		}
		if flip_horizontal {
			args.push("--hflip");
		}
		let child = Command::new("rpicam-vid")
			.args(&args)
			.stdout(Stdio::piped())
			.stderr(Stdio::null())
			.spawn()
			.expect("failed to spawn rpicam-vid");

		println!("PiCamera: rpicam-vid {}x{} MJPEG @ {}fps", width, height, framerate);

		Self {
			child,
			pending: Vec::with_capacity(READ_CHUNK * 4),
			frame_buf: Vec::new(),
			width,
			height,
		}
	}

	fn read_more(&mut self) -> Result<(), std::io::Error> {
		let stdout = self.child.stdout.as_mut()
			.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::BrokenPipe, "rpicam-vid stdout closed"))?;

		let start = self.pending.len();
		self.pending.resize(start + READ_CHUNK, 0);
		let n = stdout.read(&mut self.pending[start..])?;
		self.pending.truncate(start + n);

		if n == 0 {
			return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "rpicam-vid exited"));
		}
		Ok(())
	}

	fn find_marker(buf: &[u8], marker: &[u8; 2]) -> Option<usize> {
		buf.windows(2).position(|w| w == marker)
	}
}

impl Camera for PiCamera {
	type Error = std::io::Error;

	fn capture(&mut self) -> Result<Frame<'_>, Self::Error> {
		let soi = loop {
			if let Some(pos) = Self::find_marker(&self.pending, &JPEG_SOI) {
				if pos > 0 {
					self.pending.drain(..pos);
				}
				break 0;
			}
			self.read_more()?;
		};

		let eoi = loop {
			if let Some(pos) = Self::find_marker(&self.pending[soi + 2..], &JPEG_EOI) {
				break soi + 2 + pos + 2;
			}
			self.read_more()?;
		};

		self.frame_buf.clear();
		self.frame_buf.extend_from_slice(&self.pending[..eoi]);
		self.pending.drain(..eoi);

		Ok(Frame {
			data: &self.frame_buf,
			width: self.width,
			height: self.height,
		})
	}
}

impl Drop for PiCamera {
	fn drop(&mut self) {
		let _ = self.child.kill();
		let _ = self.child.wait();
	}
}
