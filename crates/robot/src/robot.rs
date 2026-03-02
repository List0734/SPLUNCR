use nalgebra::Vector3;

use crate::{control::{estimator::Estimators, regulator::Regulators}, data::{condition::ConfigBundle, transport::{communication::Communication, telemetry::Telemetry}}, hardware::{interface::Hal, peripheral::Peripherals, subsystem::Subsystems}, platform::{F, subsystem::propulsion::NUM_THRUSTERS}};

pub struct Robot<H: Hal> {
    communication: Communication,
    estimators: Estimators,
    subsystems: Subsystems,
    regulators: Regulators,
    peripherals: Peripherals<H>,
    telemetry: Telemetry,
}

impl<H: Hal> Robot<H> {
    pub fn new(config: ConfigBundle, peripherals: Peripherals<H>) -> Self {
        println!("Initializing Robot...");
        let telemetry = Telemetry::new();

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
        // Temporary: sine wave thrust test (~5 second period)
        let elapsed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let phase = (elapsed % (2.0 * std::f64::consts::PI * 5.0)) as f32;
        let thrust = (phase / 5.0).sin() * 0.5 + 0.5;
        let commanded = [thrust; NUM_THRUSTERS];
        self.regulators.propulsion.thruster.update(&commanded, 0.1);

        //self.estimators.odometry.apply_linear_acceleration(Vector3::new(1.0, 0.0, 0.0), 0.001);
        self.estimators.odometry.update_angular_velocity(Vector3::new(1.0, 0.0, 0.0));
        self.estimators.odometry.update(0.01);

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