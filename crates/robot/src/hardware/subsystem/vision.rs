use std::time::{Instant, SystemTime, UNIX_EPOCH};

use framework::hardware::interface::{Camera, Datagram};

const CHUNK_SIZE: usize = 60_000;
const HEADER_SIZE: usize = 20;

pub struct VisionSubsystem<C: Camera, D: Datagram> {
	camera: C,
	transport: D,
	frame_id: u32,
	last_fps_print: Instant,
	fps_count: u32,
}

impl<C: Camera, D: Datagram> VisionSubsystem<C, D> {
	pub fn new(camera: C, transport: D) -> Self {
		Self {
			camera,
			transport,
			frame_id: 0,
			last_fps_print: Instant::now(),
			fps_count: 0,
		}
	}

	pub fn capture_and_send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		let frame = self.camera.capture().map_err(|e| format!("{e:?}"))?;

		let timestamp_us = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_micros() as u64;

		let total_fragments = ((frame.data.len() + CHUNK_SIZE - 1) / CHUNK_SIZE) as u16;

		for (i, chunk) in frame.data.chunks(CHUNK_SIZE).enumerate() {
			let mut packet = Vec::with_capacity(HEADER_SIZE + chunk.len());
			packet.extend_from_slice(&self.frame_id.to_be_bytes());
			packet.extend_from_slice(&(i as u16).to_be_bytes());
			packet.extend_from_slice(&total_fragments.to_be_bytes());
			packet.extend_from_slice(&frame.width.to_be_bytes());
			packet.extend_from_slice(&frame.height.to_be_bytes());
			packet.extend_from_slice(&timestamp_us.to_be_bytes());
			packet.extend_from_slice(chunk);
			self.transport.send(&packet).map_err(|e| format!("{e:?}"))?;
		}

		self.frame_id = self.frame_id.wrapping_add(1);
		self.fps_count += 1;
		let elapsed = self.last_fps_print.elapsed();
		if elapsed.as_secs() >= 5 {
			let fps = self.fps_count as f64 / elapsed.as_secs_f64();
			println!("Vision: {:.1} fps, {} bytes/frame, {} fragments",
				fps, frame.data.len(), total_fragments);
			self.fps_count = 0;
			self.last_fps_print = Instant::now();
		}
		Ok(())
	}
}
