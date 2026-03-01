use nalgebra::RealField;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct SlewRateLimiterConfig<S> {
    pub rising_rate: S,
    pub falling_rate: S,
}

pub struct SlewRateLimiter<S> {
    config: SlewRateLimiterConfig<S>,
    current: S,
}

impl<S: RealField + Copy> SlewRateLimiter<S> {
    pub fn new(config: SlewRateLimiterConfig<S>) -> Self {
        Self {
            config,
            current: S::zero(),
        }
    }

    pub fn update(&mut self, target: S, dt: S) -> S {
        let diff = target - self.current;

        let rate = if target.abs() > self.current.abs() {
            self.config.rising_rate
        } else {
            self.config.falling_rate
        };

        let max_step = rate * dt;

        if diff.abs() <= max_step {
            self.current = target;
        } else {
            self.current = self.current + diff.signum() * max_step;
        }

        self.current
    }

    pub fn set_config(&mut self, config: SlewRateLimiterConfig<S>) {
        self.config = config;
    }

    pub fn reset(&mut self) {
        self.current = S::zero();
    }

    pub fn current(&self) -> S {
        self.current
    }
}
