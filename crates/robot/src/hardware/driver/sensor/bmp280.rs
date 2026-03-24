use rppal::i2c::I2c;

use framework::hardware::interface::{Sensor, Thermometer, Barometer};

const ADDR: u16 = 0x76;
const CHIP_ID_REG: u8 = 0xD0;
const EXPECTED_CHIP_ID: u8 = 0x58;
const CTRL_MEAS_REG: u8 = 0xF4;
const CONFIG_REG: u8 = 0xF5;
const CALIB_REG: u8 = 0x88;
const CALIB_LEN: usize = 26;
const DATA_REG: u8 = 0xF7;
const DATA_LEN: usize = 6;

struct Trimming {
	t1: u16, t2: i16, t3: i16,
	p1: u16, p2: i16, p3: i16, p4: i16, p5: i16,
	p6: i16, p7: i16, p8: i16, p9: i16,
}

impl Trimming {
	fn from_bytes(b: &[u8; CALIB_LEN]) -> Self {
		Self {
			t1: u16::from_le_bytes([b[0], b[1]]),
			t2: i16::from_le_bytes([b[2], b[3]]),
			t3: i16::from_le_bytes([b[4], b[5]]),
			p1: u16::from_le_bytes([b[6], b[7]]),
			p2: i16::from_le_bytes([b[8], b[9]]),
			p3: i16::from_le_bytes([b[10], b[11]]),
			p4: i16::from_le_bytes([b[12], b[13]]),
			p5: i16::from_le_bytes([b[14], b[15]]),
			p6: i16::from_le_bytes([b[16], b[17]]),
			p7: i16::from_le_bytes([b[18], b[19]]),
			p8: i16::from_le_bytes([b[20], b[21]]),
			p9: i16::from_le_bytes([b[22], b[23]]),
		}
	}
}

pub struct Bmp280 {
	i2c: I2c,
	trim: Trimming,
	t_fine: f32,
}

impl Bmp280 {
	pub fn new() -> Result<Self, rppal::i2c::Error> {
		let mut i2c = I2c::new()?;
		i2c.set_slave_address(ADDR)?;

		let chip_id = i2c.smbus_read_byte(CHIP_ID_REG)?;
		assert_eq!(chip_id, EXPECTED_CHIP_ID, "BMP280 not found, got 0x{chip_id:02X}");

		let mut buf = [0u8; CALIB_LEN];
		i2c.block_read(CALIB_REG, &mut buf)?;
		let trim = Trimming::from_bytes(&buf);

		i2c.smbus_write_byte(CONFIG_REG, 0x00)?;
		// Normal mode, temp oversample x2, pressure oversample x16
		i2c.smbus_write_byte(CTRL_MEAS_REG, 0b010_101_11)?;

		Ok(Self { i2c, trim, t_fine: 0.0 })
	}

	fn read_raw(&mut self) -> Result<(i32, i32), rppal::i2c::Error> {
		let mut buf = [0u8; DATA_LEN];
		self.i2c.block_read(DATA_REG, &mut buf)?;

		let pressure = ((buf[0] as i32) << 12) | ((buf[1] as i32) << 4) | ((buf[2] as i32) >> 4);
		let temp = ((buf[3] as i32) << 12) | ((buf[4] as i32) << 4) | ((buf[5] as i32) >> 4);

		Ok((temp, pressure))
	}

	// BMP280 datasheet compensation formulas (floating point variant)
	fn compensate_temp(&mut self, raw: f32) -> f32 {
		let offset = raw / 16384.0 - self.trim.t1 as f32 / 1024.0;
		let linear = offset * self.trim.t2 as f32;

		let fine_offset = raw / 131072.0 - self.trim.t1 as f32 / 8192.0;
		let quadratic = fine_offset * fine_offset * self.trim.t3 as f32;

		self.t_fine = linear + quadratic;
		(self.t_fine / 5120.0).clamp(-40.0, 85.0)
	}

	fn compensate_pressure(&self, raw: f32) -> f32 {
		let t = &self.trim;

		let thermal_offset = self.t_fine / 2.0 - 64000.0;

		let correction = thermal_offset * thermal_offset * t.p6 as f32 / 32768.0
			+ thermal_offset * t.p5 as f32 * 2.0;
		let correction = correction / 4.0 + t.p4 as f32 * 65536.0;

		let sensitivity = (t.p3 as f32 * thermal_offset * thermal_offset / 524288.0
			+ t.p2 as f32 * thermal_offset) / 524288.0;
		let sensitivity = (1.0 + sensitivity / 32768.0) * t.p1 as f32;

		if sensitivity == 0.0 {
			return 0.0;
		}

		let mut pressure = (1048576.0 - raw - correction / 4096.0) * 6250.0 / sensitivity;

		let nonlinear = t.p9 as f32 * pressure * pressure / 2147483648.0
			+ t.p8 as f32 * pressure;
		pressure += (nonlinear / 32768.0) + t.p7 as f32;

		(pressure / 16.0).clamp(30000.0, 110000.0)
	}
}

impl Sensor for Bmp280 {
	type Error = rppal::i2c::Error;

	fn calibrate(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl Thermometer<f32> for Bmp280 {
	fn read_temperature(&mut self) -> Result<f32, Self::Error> {
		let (raw_temp, _) = self.read_raw()?;
		Ok(self.compensate_temp(raw_temp as f32))
	}
}

impl Barometer<f32> for Bmp280 {
	fn read_pressure(&mut self) -> Result<f32, Self::Error> {
		let (raw_temp, raw_pressure) = self.read_raw()?;
		self.compensate_temp(raw_temp as f32);
		Ok(self.compensate_pressure(raw_pressure as f32))
	}
}
