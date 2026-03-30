use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnvironmentConfig {
    pub water_temperature: f32,
    pub water_density: f32,
    pub surface_pressure: f32,
}
