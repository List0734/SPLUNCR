use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use shared::data::transport::message::Message;



pub struct Mapper;

impl Mapper {
    pub fn ingest(robot: &mut RobotCondition, message: Message<State>) {
        let mut state = robot.state;
        let mut estimator = state.estimator;
        //let mut regulator = state.regulator;
        match message.payload {
            State::OdometryEstimator(state) => estimator.odometry = state,
            _ => {}
        }
        println!("{:?}", robot);
    }
}