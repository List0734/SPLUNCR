use std::path::PathBuf;
use std::env;

use nalgebra::Vector3;

use crate::{control::{estimator::Estimators, regulator::Regulators}, data::{condition::ConfigBundle, transport::{communication::Communication, telemetry::{self, Telemetry}}}, hardware::{interface::Hal, peripheral::Peripherals, subsystem::Subsystems}};

pub struct Robot<H: Hal> {
    communication: Communication,
    estimators: Estimators,
    subsystems: Subsystems,
    regulators: Regulators,
    peripherals: Peripherals<H>,
    telemetry: Telemetry,
}

impl<H: Hal> Robot<H> {
    pub fn new(peripherals: Peripherals<H>) -> Self {
        println!("Initializing Robot...");
        let telemetry = Telemetry::new();

        let path: PathBuf = if cfg!(feature = "simulation") {
            // Simulation: load config from the crate
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml")
        } else {
            // Run/deployed: load config next to the binary
            let mut exe_path = env::current_exe().expect("cannot get exe path");
            exe_path.pop(); // remove the binary name
            exe_path.push("config.toml");
            exe_path
        };

        let config = ConfigBundle::load(&path);
        println!("Configuration loaded from {:?}", path);

        Self {
            communication: Communication::new(config.communication).expect("Failed to establish communication."),
            estimators: Estimators::new(telemetry.publisher()),
            subsystems: Subsystems::new(config.subsystem),
            regulators: Regulators::new(config.regulator, telemetry.publisher()),
            peripherals,
            telemetry,
        }
    }

    pub fn run(&mut self) {
        //self.estimators.odometry.apply_linear_acceleration(Vector3::new(1.0, 0.0, 0.0), 0.1);
        //self.estimators.odometry.update_angular_velocity(Vector3::new(1.0, 1.0, 0.0));
        //self.estimators.odometry.update(0.01);

        /*
        self.regulators.propulsion.velocity.set_setpoint();

        self.regulators.propulsion.velocity.update(measured, dt)

        let thrusts = self.subsystems.propulsion.calculate_thrusts(wrench);
        self.subsystems.propulsion.apply_thrusts(thrusts);
        */

        //self.regulators.propulsion.velocity.set_setpoint(Vector3::new(1.0, 0.0, 0.0));
        while let Some(message) = self.telemetry.receive() {
            let bytes = bincode::serialize(&message).unwrap();
            self.communication.telemetry.send(&bytes).expect("Failed something");
        }
    }
}