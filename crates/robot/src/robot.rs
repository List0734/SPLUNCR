use std::sync::{Arc, Mutex};

use nalgebra::{Rotation3, UnitQuaternion, Vector3};

use shared::telemetry::Telemetry;
use shared::physics::kinematics::Pose;
use shared::robot::subsystem;
use crate::subsystem::{Odometry, Propulsion};

pub struct Robot {
    pub telemetry: Telemetry,
    pub propulsion: Arc<Mutex<Propulsion>>,
    pub odometry: Arc<Mutex<Odometry>>,
}

impl Robot {
    pub fn new() -> Self {
        let telemetry = Telemetry::new();

        let subsystem_publisher = telemetry.create_publisher("subsystem");

        let subsystem_base = subsystem::Base::new(&telemetry);

        let propulsion = Arc::new(Mutex::new(Propulsion::new(&subsystem_base)));

        let odometry = Arc::new(Mutex::new(Odometry::new(&subsystem_base)));
        
        Self {
            telemetry,
            propulsion,
            odometry,
        }
    }
    
    pub fn telemetry(&self) -> &Telemetry {
        &self.telemetry
    }    
}