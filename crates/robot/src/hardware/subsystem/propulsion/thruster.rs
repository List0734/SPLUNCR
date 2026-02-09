use shared::physics::{dynamics::Force, kinematics::Placement};

use crate::{data::condition::config::{Config, subsystem::propulsion::ThrusterConfig}, platform::F};

pub struct Thruster {
    placement: Placement<F>,
    //motor: Motor,
}

impl Thruster {
    pub fn new(config: ThrusterConfig) -> Self {
        let placement = Placement::from_arrays(config.placement.position, config.placement.direction);

        Self {
            placement,
        }
    }

    pub fn placement(&self) -> &Placement<F> {
        &self.placement
    }

    pub fn set_thrust(&mut self, thrust: Force<F>) {
        
    }
}

impl Config<ThrusterConfig> for Thruster {
    fn update_config(&mut self, config: ThrusterConfig) {
        //self.config = config;
        self.placement = Placement::from_arrays(config.placement.position, config.placement.direction);
    }
}