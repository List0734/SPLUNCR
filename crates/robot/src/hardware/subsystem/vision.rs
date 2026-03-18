use std::net::UdpSocket;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::Capture;
use v4l::{Format, FourCC};

use crate::data::condition::config::{VideoConfig, subsystem::vision::CameraConfig};

const CHUNK_SIZE: usize = 60_000;
// Header: frame_id(4) + frag_index(2) + total_frags(2) + width(2) + height(2) + timestamp_us(8) = 20
const HEADER_SIZE: usize = 20;

pub struct VisionSubsystem {
    stream: MmapStream<'static>,
    socket: UdpSocket,
    target_address: String,
    frame_id: u32,
    header_prefix: [u8; 4],
    last_fps_print: Instant,
    fps_count: u32,
}

impl VisionSubsystem {
    pub fn new(camera_config: CameraConfig, video_config: VideoConfig) -> Self {
        let dev = Device::with_path(&camera_config.device)
            .expect("Failed to open camera device");

        let fmt = Format::new(camera_config.width, camera_config.height, FourCC::new(b"MJPG"));
        let actual = dev.set_format(&fmt).expect("Failed to set camera format");

        use v4l::video::capture::Parameters;
        if let Ok(params) = dev.params() {
            let interval = params.interval;
            println!("Camera: {}x{} {:?} @ {}/{} fps",
                actual.width, actual.height, actual.fourcc,
                interval.denominator, interval.numerator);
        } else {
            println!("Camera: {}x{} {:?}", actual.width, actual.height, actual.fourcc);
        }

        let socket = UdpSocket::bind(&video_config.bind_address)
            .expect("Failed to bind video socket");

        let stream = MmapStream::with_buffers(&dev, Type::VideoCapture, 4)
            .expect("Failed to start camera stream");

        let mut header_prefix = [0u8; 4];
        header_prefix[0..2].copy_from_slice(&(actual.width as u16).to_be_bytes());
        header_prefix[2..4].copy_from_slice(&(actual.height as u16).to_be_bytes());

        Self {
            stream,
            socket,
            target_address: video_config.target_address,
            frame_id: 0,
            header_prefix,
            last_fps_print: Instant::now(),
            fps_count: 0,
        }
    }

    pub fn capture_and_send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (buf, _meta) = self.stream.next()?;

        let timestamp_us = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let total_fragments = ((buf.len() + CHUNK_SIZE - 1) / CHUNK_SIZE) as u16;

        for (i, chunk) in buf.chunks(CHUNK_SIZE).enumerate() {
            let mut packet = Vec::with_capacity(HEADER_SIZE + chunk.len());
            packet.extend_from_slice(&self.frame_id.to_be_bytes());
            packet.extend_from_slice(&(i as u16).to_be_bytes());
            packet.extend_from_slice(&total_fragments.to_be_bytes());
            packet.extend_from_slice(&self.header_prefix);
            packet.extend_from_slice(&timestamp_us.to_be_bytes());
            packet.extend_from_slice(chunk);
            self.socket.send_to(&packet, &self.target_address)?;
        }

        self.frame_id = self.frame_id.wrapping_add(1);
        self.fps_count += 1;
        let elapsed = self.last_fps_print.elapsed();
        if elapsed.as_secs() >= 5 {
            let fps = self.fps_count as f64 / elapsed.as_secs_f64();
            println!("Vision: {:.1} fps, {} bytes/frame, {} fragments",
                fps, buf.len(), total_fragments);
            self.fps_count = 0;
            self.last_fps_print = Instant::now();
        }
        Ok(())
    }
}
