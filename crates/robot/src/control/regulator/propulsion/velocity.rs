use nalgebra::{SVector, Vector3};
use shared::{control::controllers::{PID, PID6, pid::PIDConfig}, physics::kinematics::Twist};

use crate::data::condition::config::{Config, regulator::propulsion::VelocityConfig};

pub struct Velocity {
    pids: PID6,
    setpoint: Twist,
}

impl Velocity {
    pub fn new(config: VelocityConfig) -> Self {
        let configs: [PIDConfig; 6] = [
            config.linear.surge,
            config.linear.sway,
            config.linear.heave,
            config.angular.roll,
            config.angular.pitch,
            config.angular.yaw,
        ];

        Self {
            pids: PID6::new(configs),
            setpoint: Twist::zero(),
        }
    }

    pub fn update(&mut self, measured: &Twist, dt: f32) -> Twist {
        let setpoints: SVector<f32, 6> = SVector::from_row_slice(&[
            self.setpoint.linear.x,
            self.setpoint.linear.y,
            self.setpoint.linear.z,
            self.setpoint.angular.x,
            self.setpoint.angular.y,
            self.setpoint.angular.z,
        ]);

        let measured_vec: SVector<f32, 6> = SVector::from_row_slice(&[
            measured.linear.x,
            measured.linear.y,
            measured.linear.z,
            measured.angular.x,
            measured.angular.y,
            measured.angular.z,
        ]);

        let output = self.pids.update(&setpoints, &measured_vec, dt);

        Twist {
            linear: Vector3::new(output[0], output[1], output[2]),
            angular: Vector3::new(output[3], output[4], output[5]),
        }
    }

    pub fn set_setpoint(&mut self, setpoint: Twist) {
        self.setpoint = setpoint;
    }

    pub fn reset(&mut self) {
        self.pids.reset();
    }

    pub fn set_gains(&mut self, configs: [PIDConfig; 6]) {
        self.pids.set_gains(&configs);
    }
}

impl Config<VelocityConfig> for Velocity {
    fn update_config(&mut self, config: VelocityConfig) {
        let gains: [PIDConfig; 6] = [
            config.linear.surge,
            config.linear.sway,
            config.linear.heave,
            config.angular.roll,
            config.angular.pitch,
            config.angular.yaw,
        ];

        self.pids.set_gains(&gains);
    }
}