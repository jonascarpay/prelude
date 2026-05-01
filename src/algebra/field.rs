use crate::algebra::ring::Ring;

pub trait Field: Ring {
    /// Inverse element for `mult`
    fn recip(self) -> Self;

    fn div(self, rhs: Self) -> Self;
}
