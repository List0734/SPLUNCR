use std::sync::{Arc, Mutex};

use na::{Rotation3, UnitQuaternion, Vector3};

use crate::core::telemetry::Telemetry;
use crate::subsystem::{Odometry, Propulsion};
use crate::core::physics::kinematics::Pose;

pub struct Subsystems {
    pub propulsion: Arc<Mutex<Propulsion>>,
    pub odometry: Arc<Mutex<Odometry>>,
}

pub struct System {
    telemetry: Telemetry,
    pub subsystem: Subsystems,
}

impl System {
    pub fn new() -> Self {
        let telemetry = Telemetry::new();

        let subsystem_publisher = telemetry.create_publisher("subsystem");

        let subsystem = Subsystems {
            propulsion: Arc::new(Mutex::new(Propulsion::new(
                            subsystem_publisher.child("propulsion"),
                            [
                                Pose::from_parts(
                                    Vector3::new(
                                        5.0,
                                        5.0,
                                        5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        0.631914,
                                        0.212930,
                                        -0.631914).into()),
                                Pose::from_parts(
                                    Vector3::new(
                                        -5.0,
                                        5.0,
                                        5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        0.631914,
                                        -0.212930,
                                        0.631914).into()),
                                Pose::from_parts(
                                    Vector3::new(
                                        -5.0,
                                        -5.0,
                                        5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        1.219917,
                                        -0.908651,
                                        1.219917).into()),
                                Pose::from_parts(
                                    Vector3::new(
                                        5.0,
                                        -5.0,
                                        5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        1.219917,
                                        0.908651,
                                        -1.219917).into()),
                                Pose::from_parts(
                                    Vector3::new(
                                        5.0,
                                        5.0,
                                        -5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        -0.631914,
                                        -0.212930,
                                        -0.631914).into()),
                                Pose::from_parts(
                                    Vector3::new(
                                        -5.0,
                                        5.0,
                                        -5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        -0.631914,
                                        0.212930,
                                        0.631914).into()),
                                Pose::from_parts(
                                    Vector3::new(
                                        -5.0,
                                        -5.0,
                                        -5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        -1.219917,
                                        0.908651,
                                        1.219917).into()),
                                Pose::from_parts(
                                    Vector3::new(
                                        5.0,
                                        -5.0,
                                        -5.0).into(),
                                    UnitQuaternion::from_euler_angles(
                                        -1.219917,
                                        -0.908651,
                                        -1.219917).into()),
                            ]
                        ))),
            odometry: Arc::new(Mutex::new(Odometry::new())),
        };
        
        Self {
            telemetry,
            subsystem,
        }
    }
    
    pub fn telemetry(&self) -> &Telemetry {
        &self.telemetry
    }    
}