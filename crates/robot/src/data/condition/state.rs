pub mod estimator;
pub use estimator::EstimatorBundle;
pub mod regulator;
pub use regulator::RegulatorBundle;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct StateBundle {
    pub estimator: EstimatorBundle,
    pub regulator: RegulatorBundle,
}

impl StateBundle {
    pub fn default() -> Self {
        Self {
            estimator: EstimatorBundle::default(),
            regulator: RegulatorBundle::default(),
        }
    }
}