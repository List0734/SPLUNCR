use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use crate::subsystem::video::{Decoder, Video};

use super::context::StationContext;

pub struct VideoService {
	context: StationContext,
	video: Video,
	poll_period: Duration,
}

impl VideoService {
	pub fn new(context: StationContext, video: Video, poll_rate_hz: u32) -> Self {
		Self {
			context,
			video,
			poll_period: Duration::from_secs_f64(1.0 / poll_rate_hz as f64),
		}
	}

	pub fn run(mut self) {
		let mut decoder = Decoder::new();
		let mut buf = [0u8; 65536];

		while !self.context.shutdown.load(Ordering::Relaxed) {
			match self.video.try_receive(&mut buf) {
				Some(raw) => {
					if let Some(frame) = decoder.decode(&raw) {
						*self.context.video_frame.lock().unwrap() = Some(frame);
					}
				}
				None => thread::sleep(self.poll_period),
			}
		}
	}
}
