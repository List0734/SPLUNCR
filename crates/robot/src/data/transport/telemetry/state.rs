use crate::data::condition::state::estimator::*;

pub type State = StatePayload;

#[derive(Debug)]
pub enum StatePayload {
    // Estimators
    OdometryEstimator(OdometryEstimatorState),

    // Regulators

    // Subsystems
}