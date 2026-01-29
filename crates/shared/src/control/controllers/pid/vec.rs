use nalgebra::SVector;

use crate::control::controllers::{PID, pid::PIDConfig};

pub struct PIDVec<const N: usize> {
    axes: [PID; N],
}

impl<const N: usize> PIDVec<N> {
    pub fn new(configs: [PIDConfig; N]) -> Self {
        Self {
            axes: configs.map(PID::from_config),
        }
    }

    pub fn update(
        &mut self,
        setpoint: &SVector<f32, N>,
        measured: &SVector<f32, N>,
        dt: f32,
    ) -> SVector<f32, N> {
        SVector::from_fn(|i, _| self.axes[i].update(setpoint[i], measured[i], dt))
    }

    pub fn reset(&mut self) {
        for pid in &mut self.axes {
            pid.reset();
        }
    }

    pub fn set_gains(&mut self, configs: &[PIDConfig; N]) {
        for (pid, config) in self.axes.iter_mut().zip(configs.iter()) {
            pid.set_gains(config.kp, config.ki, config.kd);
        }
    }

    pub fn errors(&self) -> SVector<f32, N> {
        SVector::from_fn(|i, _| self.axes[i].error())
    }
}

pub type PID3 = PIDVec<3>;
pub type PID6 = PIDVec<6>;