use crate::data::video::VideoFrame;

use super::receiver::RawFrame;

pub struct Decoder {
	decoder: ffmpeg_next::codec::decoder::Video,
	scaler: Option<ffmpeg_next::software::scaling::Context>,
}

impl Decoder {
	pub fn new() -> Self {
		ffmpeg_next::init().expect("Failed to init ffmpeg");
		ffmpeg_next::log::set_level(ffmpeg_next::log::Level::Fatal);
		let codec = ffmpeg_next::codec::decoder::find(ffmpeg_next::codec::Id::MJPEG)
			.expect("MJPEG codec not found");
		let decoder = ffmpeg_next::codec::Context::new_with_codec(codec)
			.decoder()
			.video()
			.expect("Failed to open MJPEG decoder");

		Self { decoder, scaler: None }
	}

	pub fn decode(&mut self, raw: &RawFrame) -> Option<VideoFrame> {
		if raw.data.starts_with(&[0xFF, 0xD8]) {
			self.decode_mjpeg(&raw.data, raw.latency_ms)
		} else {
			Some(Self::yuyv_to_rgba(&raw.data, raw.width, raw.height, raw.latency_ms))
		}
	}

	fn decode_mjpeg(&mut self, data: &[u8], latency_ms: f32) -> Option<VideoFrame> {
		let mut packet = ffmpeg_next::Packet::copy(data);
		packet.set_pts(Some(0));
		self.decoder.send_packet(&packet).ok()?;

		let mut frame = ffmpeg_next::frame::Video::empty();
		self.decoder.receive_frame(&mut frame).ok()?;

		let w = frame.width();
		let h = frame.height();

		let sws = self.scaler.get_or_insert_with(|| {
			ffmpeg_next::software::scaling::Context::get(
				frame.format(), w, h,
				ffmpeg_next::format::Pixel::RGBA, w, h,
				ffmpeg_next::software::scaling::Flags::BILINEAR,
			).expect("Failed to create scaler")
		});

		let mut rgba = ffmpeg_next::frame::Video::empty();
		sws.run(&frame, &mut rgba).ok()?;

		let stride = rgba.stride(0);
		let pixels = if stride == w as usize * 4 {
			rgba.data(0).to_vec()
		} else {
			rgba.data(0).chunks(stride).take(h as usize)
				.flat_map(|row| &row[..w as usize * 4])
				.copied()
				.collect()
		};

		Some(VideoFrame { pixels, width: w, height: h, latency_ms })
	}

	fn yuyv_to_rgba(data: &[u8], width: u32, height: u32, latency_ms: f32) -> VideoFrame {
		let pixel_count = (width * height) as usize;
		let mut pixels = vec![0u8; pixel_count * 4];

		for i in 0..(pixel_count / 2) {
			let base = i * 4;
			let y0 = data[base] as i32;
			let u = data[base + 1] as i32 - 128;
			let y1 = data[base + 2] as i32;
			let v = data[base + 3] as i32 - 128;

			let out = i * 8;
			pixels[out]     = (y0 + ((359 * v) >> 8)).clamp(0, 255) as u8;
			pixels[out + 1] = (y0 - ((88 * u + 183 * v) >> 8)).clamp(0, 255) as u8;
			pixels[out + 2] = (y0 + ((454 * u) >> 8)).clamp(0, 255) as u8;
			pixels[out + 3] = 255;
			pixels[out + 4] = (y1 + ((359 * v) >> 8)).clamp(0, 255) as u8;
			pixels[out + 5] = (y1 - ((88 * u + 183 * v) >> 8)).clamp(0, 255) as u8;
			pixels[out + 6] = (y1 + ((454 * u) >> 8)).clamp(0, 255) as u8;
			pixels[out + 7] = 255;
		}

		VideoFrame { pixels, width, height, latency_ms }
	}
}
