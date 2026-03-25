use std::thread;
use std::time::Duration;

use nalgebra::Vector3;
use rppal::i2c::I2c;

use framework::hardware::interface::{Sensor, Thermometer, Accelerometer, Gyroscope};

const ADDR: u16 = 0x68;
const CHIP_ID_REG: u8 = 0x75;
const EXPECTED_CHIP_ID: u8 = 0x70;
const PWR_MGMT_REG: u8 = 0x6B;

const ACCEL_REG: u8 = 0x3B;
const TEMP_REG: u8 = 0x41;
const GYRO_REG: u8 = 0x43;

// +/-2g default range
const ACCEL_SCALE: f32 = 16384.0;
// +/-250 deg/s default range
const GYRO_SCALE: f32 = 131.0;

const CALIBRATION_SAMPLES: usize = 200;
const CALIBRATION_DELAY: Duration = Duration::from_millis(5);

pub struct Mpu6500 {
	i2c: I2c,
	gyro_offset: Vector3<f32>,
}

impl Mpu6500 {
	pub fn new() -> Result<Self, rppal::i2c::Error> {
		let mut i2c = I2c::new()?;
		i2c.set_slave_address(ADDR)?;

		// Wake device
		i2c.smbus_write_byte(PWR_MGMT_REG, 0x00)?;
		thread::sleep(Duration::from_millis(100));

		let chip_id = i2c.smbus_read_byte(CHIP_ID_REG)?;
		assert_eq!(chip_id, EXPECTED_CHIP_ID, "MPU6500 not found, got 0x{chip_id:02X}");

		let mut sensor = Self { i2c, gyro_offset: Vector3::zeros() };
		sensor.calibrate()?;
		Ok(sensor)
	}

	fn read_word(&mut self, reg: u8) -> Result<i16, rppal::i2c::Error> {
		let mut buf = [0u8; 2];
		self.i2c.block_read(reg, &mut buf)?;
		Ok(i16::from_be_bytes(buf))
	}

	fn read_vector(&mut self, reg: u8) -> Result<Vector3<i16>, rppal::i2c::Error> {
		let mut buf = [0u8; 6];
		self.i2c.block_read(reg, &mut buf)?;
		Ok(Vector3::new(
			i16::from_be_bytes([buf[0], buf[1]]),
			i16::from_be_bytes([buf[2], buf[3]]),
			i16::from_be_bytes([buf[4], buf[5]]),
		))
	}
}

impl Sensor for Mpu6500 {
	type Error = rppal::i2c::Error;

	fn calibrate(&mut self) -> Result<(), Self::Error> {
		let mut sum = Vector3::<f32>::zeros();

		for _ in 0..CALIBRATION_SAMPLES {
			let raw = self.read_vector(GYRO_REG)?;
			sum += raw.cast::<f32>();
			thread::sleep(CALIBRATION_DELAY);
		}

		self.gyro_offset = sum / CALIBRATION_SAMPLES as f32;
		Ok(())
	}
}

impl Accelerometer<Vector3<f32>> for Mpu6500 {
	fn read_acceleration(&mut self) -> Result<Vector3<f32>, Self::Error> {
		let raw = self.read_vector(ACCEL_REG)?;
		Ok(raw.cast::<f32>() / ACCEL_SCALE)
	}
}

impl Gyroscope<Vector3<f32>> for Mpu6500 {
	fn read_rotation(&mut self) -> Result<Vector3<f32>, Self::Error> {
		let raw = self.read_vector(GYRO_REG)?;
		Ok((raw.cast::<f32>() - self.gyro_offset) / GYRO_SCALE)
	}
}

impl Thermometer<f32> for Mpu6500 {
	fn read_temperature(&mut self) -> Result<f32, Self::Error> {
		let raw = self.read_word(TEMP_REG)?;
		Ok(raw as f32 / 340.0 + 36.53)
	}
}
