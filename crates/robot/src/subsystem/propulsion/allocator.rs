use nalgebra::{SMatrix, Vector3};
use framework::physics::dynamics::Wrench;

use framework::hardware::interface::Motor;

use crate::data::config::propulsion::MaxForce;
use crate::{subsystem::propulsion::Thruster, platform::{F, subsystem::propulsion::NUM_THRUSTERS}};

// 6 x N allocation matrix
type AllocationMatrix = SMatrix<F, 6, NUM_THRUSTERS>;

// N x 6 pseudo-inverse matrix
type PseudoInverseMatrix = SMatrix<F, NUM_THRUSTERS, 6>;

pub struct Allocator {
    pseudo_inverse: PseudoInverseMatrix,
    max_forces: [MaxForce; NUM_THRUSTERS],
    max_wrench: Wrench<F>,
}

impl Allocator {
    pub fn new<M: Motor<F>>(thrusters: &[Thruster<M>]) -> Self {
        let (allocation, max_forces) = Self::build_allocation_matrix(thrusters);
        let max_wrench = Self::compute_max_wrench(&allocation);
        let pseudo_inverse = Self::invert_allocation_matrix(allocation);
        Self { pseudo_inverse, max_forces, max_wrench }
    }

    pub fn max_wrench(&self) -> &Wrench<F> {
        &self.max_wrench
    }

    // Maps a desired wrench to per-thruster forces in Newtons.
    pub fn allocate(&self, wrench: Wrench<F>) -> [F; NUM_THRUSTERS] {
        let mut forces = self.compute_forces(wrench);
        let scale = self.find_scale_factor(&forces);
        Self::apply_scale(&mut forces, scale);
        forces
    }

    // Each column is a thruster's 6-DOF contribution: [force, torque],
    // scaled by max forward force so the pseudo-inverse produces forward-force-normalized fractions.
    fn build_allocation_matrix<M: Motor<F>>(thrusters: &[Thruster<M>]) -> (AllocationMatrix, [MaxForce; NUM_THRUSTERS]) {
        let mut allocation = AllocationMatrix::zeros();
        let mut max_forces = [MaxForce { forward: 0.0, reverse: 0.0 }; NUM_THRUSTERS];

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

            max_forces[i] = thruster.max_force();
            for row in 0..6 {
                allocation[(row, i)] *= max_forces[i].forward;
            }
        }

        (allocation, max_forces)
    }

    fn compute_max_wrench(allocation: &AllocationMatrix) -> Wrench<F> {
        let mut max = [0.0; 6];
        for row in 0..6 {
            for col in 0..NUM_THRUSTERS {
                max[row] += allocation[(row, col)].abs();
            }
        }
        Wrench {
            force: Vector3::new(max[0], max[1], max[2]),
            torque: Vector3::new(max[3], max[4], max[5]),
        }
    }

    fn invert_allocation_matrix(allocation: AllocationMatrix) -> PseudoInverseMatrix {
        allocation
            .pseudo_inverse(1e-6)
            .unwrap_or(PseudoInverseMatrix::zeros())
    }

    // Pseudo-inverse output * max_force.forward converts fractions to Newtons.
    fn compute_forces(&self, wrench: Wrench<F>) -> [F; NUM_THRUSTERS] {
        let fractions: [F; NUM_THRUSTERS] = *(&self.pseudo_inverse * wrench.as_vector()).as_ref();
        let mut forces = [0.0; NUM_THRUSTERS];
        for i in 0..NUM_THRUSTERS {
            forces[i] = fractions[i] * self.max_forces[i].forward;
        }
        forces
    }

    // Find the smallest uniform scale factor that keeps all forces within [-max_reverse, max_forward].
    fn find_scale_factor(&self, forces: &[F; NUM_THRUSTERS]) -> F {
        let mut scale = 1.0_f32;
        for i in 0..NUM_THRUSTERS {
            if forces[i] > self.max_forces[i].forward {
                scale = scale.min(self.max_forces[i].forward / forces[i]);
            } else if forces[i] < 0.0 && -forces[i] > self.max_forces[i].reverse {
                scale = scale.min(self.max_forces[i].reverse / -forces[i]);
            }
        }
        scale
    }

    fn apply_scale(forces: &mut [F; NUM_THRUSTERS], scale: F) {
        if scale < 1.0 {
            for f in forces.iter_mut() {
                *f *= scale;
            }
        }
    }
}