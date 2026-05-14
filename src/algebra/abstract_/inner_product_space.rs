use std::iter::zip;

use crate::algebra::abstract_::{additive::iter_sum, Ring, VectorSpace};

// The familiar dot product.
// Should obey <u,v> = 1/2 (Q(u+v) - Q(v) - Q(v))
// TODO relate this to ring mult multiplying magnitudes
pub trait InnerProductSpace: VectorSpace {
    // a * b.conjugate() TODO always?
    fn quadrance(self) -> Self::Scalar;
    // A bilinear operation
    fn inner(self, rhs: Self) -> Self::Scalar;
}

impl<T: Ring, const N: usize> InnerProductSpace for [T; N] {
    fn quadrance(self) -> Self::Scalar {
        iter_sum(self.into_iter().map(Ring::sq))
    }

    fn inner(self, rhs: Self) -> Self::Scalar {
        iter_sum(zip(self, rhs).map(|(a, b)| a.mult(b)))
    }
}
