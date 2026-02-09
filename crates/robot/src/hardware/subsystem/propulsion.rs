use crate::{data::condition::config::{Config, subsystem::PropulsionConfig}, platform::{F, subsystem::propulsion::NUM_THRUSTERS}};

mod thruster;
pub use thruster::Thruster;
mod allocator;
pub use allocator::Allocator;
use shared::physics::dynamics::{Force, Wrench};

pub struct PropulsionSubsystem {
    thrusters: [Thruster; NUM_THRUSTERS],
    allocator: Allocator,
}

impl PropulsionSubsystem {
    pub fn new(config: PropulsionConfig) -> Self {
        let thrusters = config.thrusters.map(Thruster::new);
        let allocator = Allocator::new(&thrusters);

        Self {
            thrusters,
            allocator,
        }
    }

    pub fn calculate_thrusts(&self, wrench: Wrench<F>) -> [Force<F>; NUM_THRUSTERS] {
        self.allocator.allocate(wrench)
    }

    pub fn apply_thrusts(&mut self, thrusts: [Force<F>; NUM_THRUSTERS]) {
        for (thruster, thrust) in self.thrusters.iter_mut().zip(thrusts) {
            thruster.set_thrust(thrust);
        }
    }
}

impl Config<PropulsionConfig> for PropulsionSubsystem {
    fn update_config(&mut self, config: PropulsionConfig) {
        for (thruster, config) in self.thrusters.iter_mut().zip(&config.thrusters) {
            thruster.update_config(config.clone());
        }

        self.allocator = Allocator::new(&self.thrusters);
    }
}