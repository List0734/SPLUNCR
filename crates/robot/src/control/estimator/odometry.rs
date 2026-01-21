use nalgebra::Vector3;

use shared::physics::kinematics::Pose;
use crate::data::transport::telemetry::{Publisher, Message};

use crate::data::condition::state::OdometryState;
//use crate::

pub struct Odometry {
    //sensors: subsystem::Sensors,
    state: OdometryState,
    //config: config::control::Odometry,
    telemetry: Publisher,
}

impl Odometry {
    pub fn new(telemetry: Publisher/*sensors: &subsystem::Sensors*/) -> Self {
        Self {
            //sensors,
            state: OdometryState::default(),
            telemetry
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.integrate(dt);
        
        self.telemetry.publish(Message::OdometryState(self.state));
    }

    pub fn integrate(&mut self, dt: f32) { 
        let delta = Pose::new(
            self.state.twist.linear * dt,
            self.state.twist.angular * dt,
        );

        self.state.pose *= delta;
    }

    pub fn apply_linear_acceleration(&mut self, acceleration: Vector3<f32>, dt: f32) {
        self.state.twist.linear += acceleration * dt;
    }

    pub fn update_angular_velocity(&mut self, velocity: Vector3<f32>) {
        self.state.twist.angular = velocity;
    }
}