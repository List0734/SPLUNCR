use crate::data::condition::config::{Config, subsystem::PropulsionConfig};

pub struct PropulsionSubsystem {
    config: PropulsionConfig,
}

impl PropulsionSubsystem {
    pub fn new(config: PropulsionConfig) -> Self {
        Self {
            config
        }
    }

    pub fn test(&self) {
        println!("{:?}", self.config);
    }
}

impl Config<PropulsionConfig> for PropulsionSubsystem {
    fn update_config(&mut self, config: PropulsionConfig) {
        self.config = config;
    }
}