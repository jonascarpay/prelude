use crate::algebra::additive::Additive;

pub trait Ring: Additive {
    /// An associative operation, distributive w.r.t. `plus`
    fn mult(self, rhs: Self) -> Self;

    /// Identity element for `mult` such that `zero() != one()`
    fn one() -> Self;
}

pub trait Field: Ring {
    /// Inverse element for `mult`
    fn recip(self) -> Self;

    fn div(self, rhs: Self) -> Self;
}
