#![allow(missing_docs)]

use downcast::Any;
use na::{DVector, Real};

use crate::detection::ColliderContactManifold;
use crate::object::BodySet;
use crate::solver::{ConstraintSet, IntegrationParameters};

/// The modeling of a contact.
pub trait ContactModel<N: Real>: Any + Send + Sync {
    /// Maximum number of velocity constraint to be generated for each contact.
    fn num_velocity_constraints(&self, manifold: &ColliderContactManifold<N>) -> usize;
    /// Generate all constraints for the given contact manifolds.
    fn constraints(
        &mut self,
        params: &IntegrationParameters<N>,
        bodies: &BodySet<N>,
        ext_vels: &DVector<N>,
        manifolds: &[ColliderContactManifold<N>],
        ground_j_id: &mut usize,
        j_id: &mut usize,
        jacobians: &mut [N],
        constraints: &mut ConstraintSet<N>,
    );

    /// Stores all the impulses found by the solver into a cache for warmstarting.
    fn cache_impulses(&mut self, constraints: &ConstraintSet<N>);
}

downcast!(<N> ContactModel<N> where N: Real);
