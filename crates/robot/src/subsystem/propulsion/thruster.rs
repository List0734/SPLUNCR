use shared::physics::kinematics::Pose;

pub struct Thruster {
    pose: Pose,
    max_thrust: f32, // Max thrust in Newtons
    thrust: f32,     // Current thrust in Newtons
    duty_cycle: f32, // Current duty cycle
}

impl Thruster {
    pub fn new(pose: Pose) -> Self {
        Self {
            pose: pose,
            thrust: 0.0,
            duty_cycle: 0.0,
            max_thrust: 0.0,
        }
    } 

    pub fn pose(&self) -> Pose {
        self.pose
    }

    pub fn command_thrust(mut self, thrust: f32) {
        self.thrust = thrust;
    }
}