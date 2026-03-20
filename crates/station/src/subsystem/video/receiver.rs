use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

const HEADER_SIZE: usize = 20;

pub struct RawFrame {
	pub data: Vec<u8>,
	pub width: u32,
	pub height: u32,
	pub latency_ms: f32,
}

pub struct FragmentReceiver {
	socket: UdpSocket,
	current_frame_id: u32,
	fragments: Vec<Option<Vec<u8>>>,
	received_count: u16,
	total_expected: u16,
	frame_width: u32,
	frame_height: u32,
	frame_timestamp_us: u64,
}

impl FragmentReceiver {
	pub fn new(socket: UdpSocket) -> Self {
		Self {
			socket,
			current_frame_id: u32::MAX,
			fragments: Vec::new(),
			received_count: 0,
			total_expected: 0,
			frame_width: 0,
			frame_height: 0,
			frame_timestamp_us: 0,
		}
	}

	pub fn try_receive(&mut self, buf: &mut [u8]) -> Option<RawFrame> {
		loop {
			let (n, _addr) = match self.socket.recv_from(buf) {
				Ok(r) => r,
				Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return None,
				Err(e) => {
					eprintln!("Video receive error: {}", e);
					return None;
				}
			};

			if n < HEADER_SIZE {
				continue;
			}

			let frame_id = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
			let fragment_index = u16::from_be_bytes([buf[4], buf[5]]);
			let total_fragments = u16::from_be_bytes([buf[6], buf[7]]);
			let width = u16::from_be_bytes([buf[8], buf[9]]) as u32;
			let height = u16::from_be_bytes([buf[10], buf[11]]) as u32;
			let timestamp_us = u64::from_be_bytes([
				buf[12], buf[13], buf[14], buf[15],
				buf[16], buf[17], buf[18], buf[19],
			]);
			let payload = &buf[HEADER_SIZE..n];

			if frame_id != self.current_frame_id {
				self.current_frame_id = frame_id;
				self.total_expected = total_fragments;
				self.fragments = vec![None; total_fragments as usize];
				self.received_count = 0;
				self.frame_width = width;
				self.frame_height = height;
				self.frame_timestamp_us = timestamp_us;
			}

			let idx = fragment_index as usize;
			if idx < self.fragments.len() && self.fragments[idx].is_none() {
				self.fragments[idx] = Some(payload.to_vec());
				self.received_count += 1;
			}

			if self.received_count == self.total_expected {
				let now_us = SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.unwrap()
					.as_micros() as u64;
				let latency_ms = now_us.saturating_sub(self.frame_timestamp_us) as f32 / 1000.0;

				let data: Vec<u8> = self.fragments
					.iter()
					.filter_map(|f| f.as_ref())
					.flat_map(|f| f.iter().copied())
					.collect();

				return Some(RawFrame {
					data,
					width: self.frame_width,
					height: self.frame_height,
					latency_ms,
				});
			}
		}
	}
}
