use nalgebra::Vector3;

use crate::{control::estimator::Estimators, data::transport::telemetry::{self, Telemetry}};

pub struct Robot {
    estimators: Estimators,
    telemetry: Telemetry,
}

impl Robot {
    pub fn new() -> Self {
        let telemetry = Telemetry::new();

        Self {
            estimators: Estimators::new(telemetry.publisher()),
            telemetry,
        }
    }

    pub fn run(&mut self) {
        //self.estimators.odometry.apply_linear_acceleration(Vector3::new(1.0, 0.0, 0.0), 0.1);
        self.estimators.odometry.update_angular_velocity(Vector3::new(1.0, 1.0, 0.0));
        self.estimators.odometry.update(0.01);
    }
    
    pub fn telemetry(&self) -> &Telemetry {
        &self.telemetry
    }    
}