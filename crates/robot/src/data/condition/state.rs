pub mod estimator;
pub use estimator::EstimatorBundle;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct StateBundle {
    pub estimator: EstimatorBundle,
}

impl StateBundle {
    pub fn default() -> Self {
        Self {
            estimator: EstimatorBundle::default(),
        }
    }
}