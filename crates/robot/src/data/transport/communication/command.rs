use serde::{Serialize, Deserialize};

use shared::physics::kinematics::Twist;

use crate::platform::F;

pub type Command = CommandPayload;

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandPayload {
    Velocity(Twist<F>),
}
