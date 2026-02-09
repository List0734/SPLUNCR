use nalgebra::Vector3;

use shared::physics::kinematics::{Pose, Twist};
use crate::data::transport::telemetry::{Publisher, Message};

use crate::data::condition::state::estimator::OdometryEstimatorState;
use crate::platform::Fp;

pub struct Odometry {
    //sensors: subsystem::Sensors,
    state: OdometryEstimatorState,
    //config: config::control::Odometry,
    telemetry: Publisher,
}

impl Odometry {
    pub fn new(telemetry: Publisher/*sensors: &subsystem::Sensors*/) -> Self {
        Self {
            //sensors,
            state: OdometryEstimatorState::default(),
            telemetry
        }
    }

    pub fn update(&mut self, dt: Fp) -> Twist<Fp> {
        self.integrate(dt); 
        self.telemetry.publish(Message::OdometryEstimator(self.state));
        self.state.twist
    }

    pub fn integrate(&mut self, dt: Fp) {
        let delta = Pose::new(
            self.state.twist.linear * dt,
            self.state.twist.angular * dt,
        );

        self.state.pose *= delta;
    }

    pub fn twist(&self) -> Twist<Fp> {
        self.state.twist
    }

    pub fn apply_linear_acceleration(&mut self, acceleration: Vector3<Fp>, dt: Fp) {
        self.state.twist.linear += acceleration * dt;
    }

    pub fn update_angular_velocity(&mut self, velocity: Vector3<Fp>) {
        self.state.twist.angular = velocity;
    }
}