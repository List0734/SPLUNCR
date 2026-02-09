use nalgebra::SMatrix;
use shared::physics::dynamics::Wrench;

use crate::{hardware::subsystem::propulsion::Thruster, platform::{F, subsystem::propulsion::NUM_THRUSTERS}};

// 6 x N allocation matrix
type AllocationMatrix = SMatrix<F, 6, NUM_THRUSTERS>;

// N x 6 pseudo-inverse matrix
type PseudoInverseMatrix = SMatrix<F, NUM_THRUSTERS, 6>;

pub struct Allocator {
    pseudo_inverse: PseudoInverseMatrix,
}

impl Allocator {
    pub fn new(thrusters: &[Thruster]) -> Self {
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

    pub fn allocate(&self, wrench: Wrench<F>) -> [F; NUM_THRUSTERS] {
        let wrench_vec = wrench.as_vector();

        let thrusts = &self.pseudo_inverse * wrench_vec;
        *thrusts.as_ref()
    }
}