use crate::data::condition::state;

pub enum Message {
    OdometryState(state::OdometryState),
//    Status(Status),
}