use nalgebra::SMatrix;
use framework::physics::dynamics::Wrench;

use framework::hardware::interface::Motor;

use crate::{hardware::subsystem::propulsion::Thruster, platform::{F, subsystem::propulsion::NUM_THRUSTERS}};

// 6 x N allocation matrix
type AllocationMatrix = SMatrix<F, 6, NUM_THRUSTERS>;

// N x 6 pseudo-inverse matrix
type PseudoInverseMatrix = SMatrix<F, NUM_THRUSTERS, 6>;

pub struct Allocator {
    pseudo_inverse: PseudoInverseMatrix,
}

impl Allocator {
    pub fn new<M: Motor<F>>(thrusters: &[Thruster<M>]) -> Self {
        let mut allocation = AllocationMatrix::zeros();

        for (i, thruster) in thrusters.iter().enumerate() {
            let placement = thruster.placement();
            let direction = placement.direction.into_inner();
            let torque = placement.position.cross(&direction);

            allocation[(0, i)] = direction.x;
            allocation[(1, i)] = direction.y;
            allocation[(2, i)] = direction.z;
            
            allocation[(3, i)] = torque.x;
            allocation[(4, i)] = torque.y;
            allocation[(5, i)] = torque.z;
        }

        let pseudo_inverse = allocation
            .pseudo_inverse(1e-6)
            .unwrap_or(PseudoInverseMatrix::zeros());

        Self { pseudo_inverse }
    }

    pub fn allocate(&self, wrench: Wrench<F>, reverse_allowed: [bool; NUM_THRUSTERS]) -> [F; NUM_THRUSTERS] {
        let wrench_vec = wrench.as_vector();
        let mut thrusts: [F; NUM_THRUSTERS] = *(&self.pseudo_inverse * wrench_vec).as_ref();

        for i in 0..NUM_THRUSTERS {
            if !reverse_allowed[i] {
                thrusts[i] = thrusts[i].max(0.0);
            }
        }

        let max_thrust = thrusts.iter().fold(0.0_f32, |m, &t| m.max(t.abs()));
        let max_input = wrench_vec.iter().fold(0.0_f32, |m, &w| m.max(w.abs())).min(1.0);

        if max_thrust > 0.0 {
            let scale = max_input / max_thrust;
            for t in &mut thrusts {
                *t *= scale;
            }
        }

        thrusts
    }
}