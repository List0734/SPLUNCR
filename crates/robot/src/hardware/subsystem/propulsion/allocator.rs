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
    reverse_limits: [F; NUM_THRUSTERS],
}

impl Allocator {
    pub fn new<M: Motor<F>>(thrusters: &[Thruster<M>]) -> Self {
        let mut allocation = AllocationMatrix::zeros();
        let mut reverse_limits = [0.0; NUM_THRUSTERS];

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

            let max_force = thruster.max_force();
            for row in 0..6 {
                allocation[(row, i)] *= max_force.forward;
            }
            reverse_limits[i] = max_force.reverse / max_force.forward;
        }

        let mut pseudo_inverse = allocation
            .pseudo_inverse(1e-6)
            .unwrap_or(PseudoInverseMatrix::zeros());

        for col in 0..6 {
            let mut max_val: F = 0.0;
            for row in 0..NUM_THRUSTERS {
                max_val = max_val.max(pseudo_inverse[(row, col)].abs());
            }
            if max_val > 0.0 {
                for row in 0..NUM_THRUSTERS {
                    pseudo_inverse[(row, col)] /= max_val;
                }
            }
        }

        Self { pseudo_inverse, reverse_limits }
    }

    pub fn allocate(&self, wrench: Wrench<F>, reverse_allowed: [bool; NUM_THRUSTERS]) -> [F; NUM_THRUSTERS] {
        let wrench_vec = wrench.as_vector();
        let mut thrusts: [F; NUM_THRUSTERS] = *(&self.pseudo_inverse * wrench_vec).as_ref();

        let mut scale = 1.0_f32;
        for i in 0..NUM_THRUSTERS {
            if thrusts[i] > 1.0 {
                scale = scale.min(1.0 / thrusts[i]);
            } else if thrusts[i] < 0.0 {
                let limit = if reverse_allowed[i] { self.reverse_limits[i] } else { 0.0 };
                if limit > 0.0 && -thrusts[i] > limit {
                    scale = scale.min(limit / -thrusts[i]);
                }
            }
        }
        if scale < 1.0 {
            for t in &mut thrusts {
                *t *= scale;
            }
        }

        for i in 0..NUM_THRUSTERS {
            if !reverse_allowed[i] {
                thrusts[i] = thrusts[i].max(0.0);
            }
        }

        thrusts
    }
}