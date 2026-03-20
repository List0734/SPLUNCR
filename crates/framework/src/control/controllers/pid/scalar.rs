use nalgebra::RealField;
use serde::{Deserialize, Serialize};

pub struct PID<S> {
    config: PIDConfig<S>,
    integral: S,
    prev_error: S,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct PIDConfig<S> {
    pub kp: S,
    pub ki: S,
    pub kd: S,
}

impl<S: RealField + Copy> PID<S> {
    pub fn new(kp: S, ki: S, kd: S) -> Self {
        Self {
            config: PIDConfig { kp, ki, kd },
            integral: S::zero(),
            prev_error: S::zero(),
        }
    }

    pub fn from_config(config: PIDConfig<S>) -> Self {
        Self {
            config,
            integral: S::zero(),
            prev_error: S::zero(),
        }
    }

    pub fn set_gains(&mut self, kp: S, ki: S, kd: S) {
        self.config.kp = kp;
        self.config.ki = ki;
        self.config.kd = kd;
    }

    pub fn update(&mut self, setpoint: S, measurement: S, dt: S) -> S {
        let PIDConfig { kp, ki, kd } = self.config;

        let error = setpoint - measurement;

        self.integral = self.integral + error * dt;

        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;

        kp * error + ki * self.integral + kd * derivative
    }

    pub fn reset(&mut self) {
        self.integral = S::zero();
        self.prev_error = S::zero();
    }

    pub fn error(&self) -> S {
        self.prev_error
    }
}