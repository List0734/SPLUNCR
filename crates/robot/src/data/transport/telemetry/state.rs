use serde::{Serialize, Deserialize};

use crate::data::condition::state::{estimator::*, regulator::*};

pub type State = StatePayload;

#[derive(Debug, Serialize, Deserialize)]
pub enum StatePayload {
    // Estimators
    OdometryEstimator(OdometryEstimatorState),

    // Regulators
    CoastRegulator(CoastRegulatorState),

    // Subsystems
}