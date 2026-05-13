use super::ring::Ring;

pub trait Field: Ring {
    /// Inverse element for `mult`
    fn recip(self) -> Self {
        Self::one().div(self)
    }

    fn div(self, rhs: Self) -> Self;
}

// TODO: rationals

impl Field for f32 {
    fn recip(self) -> Self {
        self.recip()
    }
    fn div(self, rhs: Self) -> Self {
        self / rhs
    }
}

impl Field for f64 {
    fn recip(self) -> Self {
        self.recip()
    }
    fn div(self, rhs: Self) -> Self {
        self / rhs
    }
}

// TODO impl_field_ops
