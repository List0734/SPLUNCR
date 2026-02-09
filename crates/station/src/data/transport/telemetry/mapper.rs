use robot::data::{transport::telemetry::{self, Message}, condition::RobotCondition};

pub struct Mapper;

impl Mapper {
    pub fn ingest(robot: &mut RobotCondition, message: Message) {
        let mut state = robot.state;
        let mut estimator = state.estimator;
        //let mut regulator = state.regulator;
        match message {
            Message::OdometryEstimator(state) => estimator.odometry = state,
            _ => {}
        }
        println!("{:?}", robot);
    }
}