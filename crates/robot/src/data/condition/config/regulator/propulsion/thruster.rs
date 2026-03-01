use serde::{Deserialize, Serialize};

pub mod coast;
pub use coast::CoastRegulatorConfig;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct ThrusterRegulatorConfig {
    pub coast: CoastRegulatorConfig,
}
