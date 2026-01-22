use std::path::PathBuf;

use nalgebra::Vector3;

use crate::{control::estimator::Estimators, data::{condition::ConfigBundle, transport::telemetry::{self, Telemetry}}};

pub struct Robot {
    config: ConfigBundle,
    estimators: Estimators,
    telemetry: Telemetry,
}

impl Robot {
    pub fn new() -> Self {
        let telemetry = Telemetry::new();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        if cfg!(feature = "simulation") {
            path.push("config.toml");
        } else {
            path.push("config.toml");
        }

        let config = ConfigBundle::load(path);

        Self {
            config,
            estimators: Estimators::new(telemetry.publisher()),
            telemetry,
        }
    }

    pub fn run(&mut self) {
        println!("{:?}", self.config.subsystem)

        //self.estimators.odometry.apply_linear_acceleration(Vector3::new(1.0, 0.0, 0.0), 0.1);
//        self.estimators.odometry.update_angular_velocity(Vector3::new(1.0, 1.0, 0.0));
//        self.estimators.odometry.update(0.01);
    }
    
    pub fn telemetry(&self) -> &Telemetry {
        &self.telemetry
    }    
}