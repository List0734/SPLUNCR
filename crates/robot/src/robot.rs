use std::time::{SystemTime, UNIX_EPOCH};

use nalgebra::Vector3;
use shared::data::transport::message::Message;

use crate::{control::{estimator::Estimators, regulator::Regulators}, data::{condition::ConfigBundle, transport::{communication::{Communication, command::{Command, CommandPayload}}, telemetry::Telemetry}}, hardware::{interface::{Hal, motor::Motor}, peripheral::Peripherals, subsystem::Subsystems}, platform::{F, subsystem::propulsion::NUM_THRUSTERS}};

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

    pub fn init_motors(&mut self) {
        let commanded = [0.0; NUM_THRUSTERS];
        let outputs = self.regulators.propulsion.thruster.update(&commanded, 0.01);
        for (motor, &duty) in self.peripherals.motors.iter_mut().zip(outputs.iter()) {
            motor.set_duty_cycle(duty).expect("Failed to set motor duty cycle");
        }
    }

    pub fn run(&mut self) {
        let mut cmd_buf = [0u8; 1024];
        while let Ok(Some(n)) = self.communication.commands.try_receive(&mut cmd_buf) {
            let Ok(msg) = bincode::deserialize::<Message<Command>>(&cmd_buf[..n]) else {
                eprintln!("Ignored malformed command ({n} bytes)");
                continue;
            };
            match msg.payload {
                CommandPayload::Ping => {
                    let t2 = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_micros() as u64;
                    let mut pong = [0u8; 16];
                    pong[0..8].copy_from_slice(&msg.timestamp.to_be_bytes());
                    pong[8..16].copy_from_slice(&t2.to_be_bytes());
                    let _ = self.communication.commands.send(&pong);
                }
                CommandPayload::SetClock(ts_us) => {
                    let secs = (ts_us / 1_000_000) as i64;
                    let nsecs = ((ts_us % 1_000_000) * 1000) as i64;
                    let ts = libc::timespec { tv_sec: secs, tv_nsec: nsecs };
                    let ret = unsafe { libc::clock_settime(libc::CLOCK_REALTIME, &ts) };
                    if ret == 0 {
                        println!("Clock set to {secs}.{:06}", ts_us % 1_000_000);
                    } else {
                        eprintln!("Failed to set clock: {}", std::io::Error::last_os_error());
                    }
                }
                _ => {}
            }
        }

        // Temporary: sine wave thrust test (~5 second period)
        let elapsed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let phase = (elapsed % (2.0 * std::f64::consts::PI * 5.0)) as f32;
        let thrust = (phase / 5.0).sin();
        let scaled_thrust = 0.20 * thrust;
        let commanded = [scaled_thrust; NUM_THRUSTERS];
        let outputs = self.regulators.propulsion.thruster.update(&commanded, 0.01);
        for (motor, &duty) in self.peripherals.motors.iter_mut().zip(outputs.iter()) {
            motor.set_duty_cycle(duty).expect("Failed to set motor duty cycle");
        }

        /*
        let commanded = [1.0; NUM_THRUSTERS];
        for (motor, &duty) in self.peripherals.motors.iter_mut().zip(commanded.iter()) {
            motor.set_duty_cycle(duty).expect("Failed to set motor duty cycle");
        }

        thread::sleep(Duration::from_millis(5000));

        let commanded = [-1.0; NUM_THRUSTERS];
        for (motor, &duty) in self.peripherals.motors.iter_mut().zip(commanded.iter()) {
            motor.set_duty_cycle(duty).expect("Failed to set motor duty cycle");
        }

        thread::sleep(Duration::from_millis(5000));

        let commanded = [0.0; NUM_THRUSTERS];
        for (motor, &duty) in self.peripherals.motors.iter_mut().zip(commanded.iter()) {
            motor.set_duty_cycle(duty).expect("Failed to set motor duty cycle");
        }
        
        thread::sleep(Duration::from_millis(5000));
        */

        //self.estimators.odometry.apply_linear_acceleration(Vector3::new(1.0, 0.0, 0.0), 0.001);
        //self.estimators.odometry.update_angular_velocity(Vector3::new(1.0, 0.0, 0.0));
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