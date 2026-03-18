use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use shared::data::transport::message::Message;

use crate::data::{config::StationCommunicationConfig, transport::telemetry::Mapper};

mod tcp;
mod udp;
use tcp::Tcp;
use udp::Udp;

// Header: frame_id(4) + frag_index(2) + total_frags(2) + width(2) + height(2) + timestamp_us(8) = 20
const HEADER_SIZE: usize = 20;

pub struct VideoFrame {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub latency_ms: f32,
}

pub struct Communication {
    config: StationCommunicationConfig,
    commands: Arc<Mutex<Option<Tcp>>>,
    connected: Arc<AtomicBool>,
    telemetry: Udp,
    video: Udp,
    video_frame: Arc<Mutex<Option<VideoFrame>>>,
}

impl Communication {
    pub fn new(config: StationCommunicationConfig) -> io::Result<Self> {
        let telemetry = Udp::new(&config.telemetry.listen_address)?;
        let video = Udp::new(&config.video.listen_address)?;
        Ok(Self {
            config,
            commands: Arc::new(Mutex::new(None)),
            connected: Arc::new(AtomicBool::new(false)),
            telemetry,
            video,
            video_frame: Arc::new(Mutex::new(None)),
        })
    }

    pub fn video_frame(&self) -> Arc<Mutex<Option<VideoFrame>>> {
        Arc::clone(&self.video_frame)
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    pub fn send_command(&self, data: &[u8]) -> io::Result<()> {
        let mut guard = self.commands.lock().unwrap();
        if let Some(tcp) = guard.as_mut() {
            match tcp.send(data) {
                Ok(()) => Ok(()),
                Err(e) => {
                    *guard = None;
                    self.connected.store(false, Ordering::Release);
                    Err(e)
                }
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "not connected"))
        }
    }

    pub fn spawn_command_connector(self: &Arc<Self>) -> JoinHandle<()> {
        let commands = Arc::clone(&self.commands);
        let connected = Arc::clone(&self.connected);
        let target_addr = self.config.command.target_address.clone();
        thread::spawn(move || loop {
            if connected.load(Ordering::Acquire) {
                thread::sleep(Duration::from_secs(1));
                continue;
            }
            match Tcp::connect(&target_addr) {
                Ok(mut tcp) => {
                    match Self::clock_sync(&mut tcp) {
                        Ok(()) => println!("Clock synced with robot"),
                        Err(e) => {
                            eprintln!("Clock sync failed: {e}");
                            thread::sleep(Duration::from_secs(1));
                            continue;
                        }
                    }
                    let garbage = [0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
                    let _ = tcp.send(&garbage);
                    println!("Sent garbage packet to robot");
                    *commands.lock().unwrap() = Some(tcp);
                    connected.store(true, Ordering::Release);
                }
                Err(_) => thread::sleep(Duration::from_secs(1)),
            }
        })
    }

    fn clock_sync(tcp: &mut Tcp) -> io::Result<()> {
        use robot::data::transport::communication::command::CommandPayload;

        let t1 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let ping = Message {
            timestamp: t1,
            payload: CommandPayload::Ping,
        };
        let bytes = bincode::serialize(&ping)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        tcp.send(&bytes)?;

        let mut pong = [0u8; 16];
        tcp.receive_exact(&mut pong, Duration::from_secs(5))?;

        let t4 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        let rtt = t4.saturating_sub(t1);
        let set_time = t4 + rtt / 2;

        let set_clock = Message {
            timestamp: t4,
            payload: CommandPayload::SetClock(set_time),
        };
        let bytes = bincode::serialize(&set_clock)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        tcp.send(&bytes)?;

        println!("RTT: {:.2}ms", rtt as f64 / 1000.0);
        Ok(())
    }

    pub fn spawn_telemetry_receiver(self: &Arc<Self>, robot: Arc<Mutex<RobotCondition>>) -> JoinHandle<()> {
        let communication = Arc::clone(self);
        thread::spawn(move || {
            let mut buf = [0u8; 65536];
            loop {
                match communication.telemetry.try_receive(&mut buf) {
                    Ok(Some((n, _addr))) => {
                        if let Ok(message) = bincode::deserialize::<Message<State>>(&buf[..n]) {
                            let mut robot = robot.lock().unwrap();
                            Mapper::ingest(&mut robot, message);
                        }
                    }
                    Ok(None) => thread::sleep(Duration::from_millis(1)),
                    Err(e) => eprintln!("Telemetry receive error: {}", e),
                }
            }
        })
    }

    pub fn spawn_video_receiver(self: &Arc<Self>) -> JoinHandle<()> {
        let communication = Arc::clone(self);
        thread::spawn(move || {
            ffmpeg_next::init().expect("Failed to init ffmpeg");
            let codec = ffmpeg_next::codec::decoder::find(ffmpeg_next::codec::Id::MJPEG)
                .expect("MJPEG codec not found");
            let mut decoder = ffmpeg_next::codec::Context::new_with_codec(codec)
                .decoder()
                .video()
                .expect("Failed to open MJPEG decoder");
            let mut scaler: Option<ffmpeg_next::software::scaling::Context> = None;

            let mut buf = [0u8; 65536];
            let mut current_frame_id: u32 = u32::MAX;
            let mut fragments: Vec<Option<Vec<u8>>> = Vec::new();
            let mut received_count: u16 = 0;
            let mut total_expected: u16 = 0;
            let mut frame_width: u32 = 0;
            let mut frame_height: u32 = 0;
            let mut frame_timestamp_us: u64 = 0;

            loop {
                match communication.video.try_receive(&mut buf) {
                    Ok(Some((n, _addr))) => {
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

                        if frame_id != current_frame_id {
                            current_frame_id = frame_id;
                            total_expected = total_fragments;
                            fragments = vec![None; total_fragments as usize];
                            received_count = 0;
                            frame_width = width;
                            frame_height = height;
                            frame_timestamp_us = timestamp_us;
                        }

                        let idx = fragment_index as usize;
                        if idx < fragments.len() && fragments[idx].is_none() {
                            fragments[idx] = Some(payload.to_vec());
                            received_count += 1;
                        }

                        if received_count == total_expected {
                            let now_us = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_micros() as u64;
                            let latency_ms = now_us.saturating_sub(frame_timestamp_us) as f32 / 1000.0;

                            let raw: Vec<u8> = fragments
                                .iter()
                                .filter_map(|f| f.as_ref())
                                .flat_map(|f| f.iter().copied())
                                .collect();

                            let frame = if raw.starts_with(&[0xFF, 0xD8]) {
                                decode_mjpeg(&mut decoder, &mut scaler, &raw, latency_ms)
                            } else {
                                Some(yuyv_to_rgba(&raw, frame_width, frame_height, latency_ms))
                            };

                            if let Some(frame) = frame {
                                *communication.video_frame.lock().unwrap() = Some(frame);
                            }
                        }
                    }
                    Ok(None) => thread::sleep(Duration::from_millis(1)),
                    Err(e) => eprintln!("Video receive error: {}", e),
                }
            }
        })
    }
}

fn decode_mjpeg(
    decoder: &mut ffmpeg_next::codec::decoder::Video,
    scaler: &mut Option<ffmpeg_next::software::scaling::Context>,
    data: &[u8],
    latency_ms: f32,
) -> Option<VideoFrame> {
    let mut packet = ffmpeg_next::Packet::copy(data);
    packet.set_pts(Some(0));
    decoder.send_packet(&packet).ok()?;

    let mut frame = ffmpeg_next::frame::Video::empty();
    decoder.receive_frame(&mut frame).ok()?;

    let w = frame.width();
    let h = frame.height();

    let sws = scaler.get_or_insert_with(|| {
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
