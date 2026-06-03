use super::ring::Ring;

pub trait Field: Ring {
    /// Inverse element for `mult`.
    /// Behavior when self.is_zero() is undefined.
    fn recip(self) -> Self {
        Self::one().div(self)
    }

    fn div(self, rhs: Self) -> Self;

    fn checked_div(self, rhs: Self) -> Option<Self>;

    fn checked_recip(self) -> Option<Self> {
        Self::one().checked_div(self)
    }
}

// TODO: rationals

impl Field for f32 {
    fn recip(self) -> Self {
        self.recip()
    }
    fn div(self, rhs: Self) -> Self {
        self / rhs
    }
    fn checked_recip(self) -> Option<Self> {
        let res = self.recip();
        res.is_finite().then_some(res)
    }
    fn checked_div(self, rhs: Self) -> Option<Self> {
        let res = self / rhs;
        res.is_finite().then_some(res)
    }
}

impl Field for f64 {
    fn recip(self) -> Self {
        self.recip()
    }
    fn div(self, rhs: Self) -> Self {
        self / rhs
    }
    fn checked_recip(self) -> Option<Self> {
        let res = self.recip();
        res.is_finite().then_some(res)
    }
    fn checked_div(self, rhs: Self) -> Option<Self> {
        let res = self / rhs;
        res.is_finite().then_some(res)
    }
}

// TODO impl_field_ops
