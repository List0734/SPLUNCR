use serde::Deserialize;

pub struct PID {
    config: PIDConfig,
    integral: f32,
    prev_error: f32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct PIDConfig {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

impl PID {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            config: PIDConfig { kp, ki, kd },
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    pub fn from_config(config: PIDConfig) -> Self {
        Self {
            config,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    pub fn set_gains(&mut self, kp: f32, ki: f32, kd: f32) {
        self.config.kp = kp;
        self.config.ki = ki;
        self.config.kd = kd;
    }

    pub fn update(&mut self, setpoint: f32, measurement: f32, dt: f32) -> f32 {
        let PIDConfig { kp, ki, kd } = self.config;

        let error = setpoint - measurement;

        self.integral = self.integral + error * dt;

        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;

        kp * error + ki * self.integral + kd * derivative
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
    }

    pub fn error(&self) -> f32 {
        self.prev_error
    }
}