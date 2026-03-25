pub trait Sensor {
	type Error: core::fmt::Debug;
	fn calibrate(&mut self) -> Result<(), Self::Error>;
}

pub trait Thermometer<S>: Sensor {
	fn read_temperature(&mut self) -> Result<S, Self::Error>;
}

pub trait Barometer<S>: Sensor {
	fn read_pressure(&mut self) -> Result<S, Self::Error>;
}

pub trait Accelerometer<R>: Sensor {
	fn read_acceleration(&mut self) -> Result<R, Self::Error>;
}

pub trait Gyroscope<R>: Sensor {
	fn read_rotation(&mut self) -> Result<R, Self::Error>;
}

pub trait Bathometer<S>: Sensor {
	fn read_depth(&mut self) -> Result<S, Self::Error>;
}
