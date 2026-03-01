use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use shared::data::transport::message::Message;



pub struct Mapper;

impl Mapper {
    pub fn ingest(robot: &mut RobotCondition, message: Message<State>) {
        match message.payload {
            State::OdometryEstimator(odometry) => robot.state.estimator.odometry = odometry,
            State::CoastRegulator(coast) => robot.state.regulator.propulsion.thruster.coast = coast,
        }
    }
}