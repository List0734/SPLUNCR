use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;

use framework::hardware::interface::{Sensor, Thermometer, Barometer, Bathometer};

const ADDR: u16 = 0x76;
const CMD_RESET: u8 = 0x1E;
const CMD_ADC_READ: u8 = 0x00;
const PROM_BASE: u8 = 0xA0;
const PROM_LEN: usize = 7;
const RESET_DELAY: Duration = Duration::from_millis(10);

const OVERSAMPLING: u8 = 4; // OSR 4096
const CMD_CONVERT_PRESSURE: u8 = 0x40 + 2 * OVERSAMPLING;
const CMD_CONVERT_TEMPERATURE: u8 = 0x50 + 2 * OVERSAMPLING;
const CONVERSION_DELAY: Duration = Duration::from_millis(10);

// Second order compensation thresholds (centidegrees)
const COLD_THRESHOLD: i64 = 2000;    // 20.00 °C
const FRIGID_THRESHOLD: i64 = -1500; // -15.00 °C

const SURFACE_PRESSURE_MBAR: f32 = 1013.25;
const SEAWATER_DENSITY: f32 = 1029.0; // kg/m³
const GRAVITY: f32 = 9.80665; // m/s²

pub struct Ms5837 {
	i2c: I2c,
	cal: [u16; PROM_LEN],
	pressure_mbar: f32,
	temperature_c: f32,
}

impl Ms5837 {
	pub fn new() -> Result<Self, rppal::i2c::Error> {
		let mut i2c = I2c::new()?;
		i2c.set_slave_address(ADDR)?;

		i2c.smbus_write_byte(CMD_RESET, 0x00)?;
		thread::sleep(RESET_DELAY);

		let mut cal = [0u16; PROM_LEN];
		for i in 0..PROM_LEN {
			let word = i2c.smbus_read_word(PROM_BASE + 2 * i as u8)?;
			cal[i] = word.swap_bytes();
		}

		Ok(Self { i2c, cal, pressure_mbar: 0.0, temperature_c: 0.0 })
	}

	fn read_adc(&mut self, command: u8) -> Result<u32, rppal::i2c::Error> {
		self.i2c.smbus_write_byte(command, 0x00)?;
		thread::sleep(CONVERSION_DELAY);

		let mut buf = [0u8; 3];
		self.i2c.block_read(CMD_ADC_READ, &mut buf)?;
		Ok((buf[0] as u32) << 16 | (buf[1] as u32) << 8 | buf[2] as u32)
	}

	// MS5837-30BA datasheet compensation
	fn convert(&mut self) -> Result<(), rppal::i2c::Error> {
		let d1 = self.read_adc(CMD_CONVERT_PRESSURE)? as i64;
		let d2 = self.read_adc(CMD_CONVERT_TEMPERATURE)? as i64;
		let c = |i: usize| self.cal[i] as i64;

		let dt = d2 - c(5) * (1 << 8); // difference from reference temperature

		let mut temp = COLD_THRESHOLD + dt * c(6) / (1 << 23); // centidegrees
		let mut sens = c(1) * (1 << 15) + c(3) * dt / (1 << 8); // fixed-point sensitivity
		let mut off  = c(2) * (1 << 16) + c(4) * dt / (1 << 7); // fixed-point offset

		// Second order corrections
		if temp < COLD_THRESHOLD {
			let ti    = 3 * dt * dt / (1i64 << 33);
			let offi  = 3 * (temp - COLD_THRESHOLD).pow(2) / 2;
			let sensi = 5 * (temp - COLD_THRESHOLD).pow(2) / 8;

			let (offi, sensi) = if temp < FRIGID_THRESHOLD {
				(
					offi  + 7 * (temp - FRIGID_THRESHOLD).pow(2),
					sensi + 4 * (temp - FRIGID_THRESHOLD).pow(2),
				)
			} else {
				(offi, sensi)
			};

			temp -= ti;
			off  -= offi;
			sens -= sensi;
		} else {
			temp -= 2 * dt * dt / (1i64 << 37);
			off  -= (temp - COLD_THRESHOLD).pow(2) / 16;
		}

		let pressure = (d1 * sens / (1 << 21) - off) / (1 << 13); // fixed-point -> raw

		self.temperature_c = temp as f32 / 100.0; // centidegrees -> °C
		self.pressure_mbar = pressure as f32 / 10.0; // raw -> mbar
		Ok(())
	}
}

impl Sensor for Ms5837 {
	type Error = rppal::i2c::Error;

	fn calibrate(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

impl Thermometer<f32> for Ms5837 {
	fn read_temperature(&mut self) -> Result<f32, Self::Error> {
		self.convert()?;
		Ok(self.temperature_c)
	}
}

impl Barometer<f32> for Ms5837 {
	fn read_pressure(&mut self) -> Result<f32, Self::Error> {
		self.convert()?;
		Ok(self.pressure_mbar)
	}
}

impl Bathometer<f32> for Ms5837 {
	fn read_depth(&mut self) -> Result<f32, Self::Error> {
		self.convert()?;
		let gauge_pa = (self.pressure_mbar - SURFACE_PRESSURE_MBAR) * 100.0; // mbar -> Pa
		Ok(gauge_pa / (SEAWATER_DENSITY * GRAVITY)) // Pa -> meters
	}
}
