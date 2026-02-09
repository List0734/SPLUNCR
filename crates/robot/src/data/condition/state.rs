pub mod estimator;
pub use estimator::EstimatorBundle;

#[derive(Clone, Copy, Debug)]
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