mod subsystem;
use std::{fs, io};

use serde::Deserialize;
pub use subsystem::SubsystemConfig;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct ConfigBundle {
    pub subsystem: SubsystemConfig,
}

impl ConfigBundle {
    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Self {
            let path_ref = path.as_ref();
            let raw = fs::read_to_string(path_ref).unwrap_or_else(|e| {
            eprintln!("Failed to read config file {:?}: {}", path_ref, e);
            std::process::exit(1);
        });

        toml::from_str(&raw).unwrap_or_else(|e| {
            eprintln!("Failed to parse config file {:?}: {}", path_ref, e);
            std::process::exit(1);
        })
    }
}

pub enum ConfigError {
    Io(io::Error),
    Parse(toml::de::Error),
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::Parse(e)
    }
}