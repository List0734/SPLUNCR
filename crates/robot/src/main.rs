use robot::{control::estimator, data::transport::telemetry::{self, Telemetry}};



fn main() {
    let telemetry = Telemetry::new();

    let mut odometry = estimator::Odometry::new(telemetry.publisher());

    std::thread::spawn({
        let telemetry = telemetry; // move Telemetry into thread
        move || {
            while let Some(msg) = telemetry.receive() {
                match msg {
                    telemetry::Message::OdometryEstimator(state) => println!("Odometry: {:?}", state),
                    _ => {}
                }
            }
        }
    });

    loop {
        odometry.update(0.01);

        //telemetry.receive().unwrap();
    }
}