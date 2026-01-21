use robot::data::{transport::telemetry::{self, Message}, condition::RobotCondition};

pub struct Mapper;

impl Mapper {
    pub fn ingest(robot: &mut RobotCondition, message: Message) {
        match message {
            Message::OdometryState(state) => robot.state.odometry = state,
            _ => {}
        }
        println!("{:?}", robot);
    }
}