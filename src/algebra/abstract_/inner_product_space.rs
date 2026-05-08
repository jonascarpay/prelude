use crate::algebra::abstract_::VectorSpace;

// The familiar dot product.
// Should obey <u,v> = 1/2 (Q(u+v) - Q(v) - Q(v))
pub trait InnerProductSpace: VectorSpace {
    // a * b.conjugate() TODO always?
    fn quadrance(self) -> Self::Over;
    // A bilinear operation
    fn inner(self, rhs: Self) -> Self::Over;
}
