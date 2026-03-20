use nalgebra::{RealField, SVector};

use crate::control::controllers::{PID, pid::PIDConfig};

pub struct PIDVec<S, const N: usize> {
    axes: [PID<S>; N],
}

impl<S, const N: usize> PIDVec<S, N> 
where
    S: RealField + Copy,
{
    pub fn new(configs: [PIDConfig<S>; N]) -> Self {
        Self {
            axes: configs.map(PID::from_config),
        }
    }

    pub fn update(
        &mut self,
        setpoint: &SVector<S, N>,
        measured: &SVector<S, N>,
        dt: S,
    ) -> SVector<S, N> {
        SVector::from_fn(|i, _| self.axes[i].update(setpoint[i], measured[i], dt))
    }

    pub fn reset(&mut self) {
        for pid in &mut self.axes {
            pid.reset();
        }
    }

    pub fn set_gains(&mut self, configs: &[PIDConfig<S>; N]) {
        for (pid, config) in self.axes.iter_mut().zip(configs.iter()) {
            pid.set_gains(config.kp, config.ki, config.kd);
        }
    }

    pub fn errors(&self) -> SVector<S, N> {
        SVector::from_fn(|i, _| self.axes[i].error())
    }
}

pub type PID3<S> = PIDVec<S, 3>;
pub type PID6<S> = PIDVec<S, 6>;