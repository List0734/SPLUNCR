use shared::control::controllers::slew_rate_limiter::{SlewRateLimiter, SlewRateLimiterConfig};

use crate::{
    data::{
        condition::{config::{Config, regulator::propulsion::thruster::CoastConfig}, state::regulator::CoastRegulatorState},
        transport::telemetry::{Publisher, state::State},
    },
    platform::{F, subsystem::propulsion::NUM_THRUSTERS},
};

pub struct CoastRegulator {
    limiters: [SlewRateLimiter<F>; NUM_THRUSTERS],
    telemetry: Publisher,
}

impl CoastRegulator {
    pub fn new(config: CoastConfig, telemetry: Publisher) -> Self {
        let limiter_config = SlewRateLimiterConfig {
            rising_rate: config.accel_rate,
            falling_rate: config.decel_rate,
        };

        Self {
            limiters: [limiter_config; NUM_THRUSTERS].map(SlewRateLimiter::new),
            telemetry,
        }
    }

    pub fn update(&mut self, commanded: &[F; NUM_THRUSTERS], dt: F) -> [F; NUM_THRUSTERS] {
        let mut output = [0.0; NUM_THRUSTERS];
        for (i, limiter) in self.limiters.iter_mut().enumerate() {
            output[i] = limiter.update(commanded[i], dt);
        }

        self.telemetry.publish(State::CoastRegulator(CoastRegulatorState {
            commanded: *commanded,
            output,
        }));

        output
    }
}

impl Config<CoastConfig> for CoastRegulator {
    fn update_config(&mut self, config: CoastConfig) {
        let limiter_config = SlewRateLimiterConfig {
            rising_rate: config.accel_rate,
            falling_rate: config.decel_rate,
        };

        for limiter in &mut self.limiters {
            limiter.set_config(limiter_config);
        }
    }
}
