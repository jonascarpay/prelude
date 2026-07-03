use crate::algebra::abstract_::field::Field;

/// A bit of a grab bag of functions that can't live in weaker traits.
pub trait Real: Field + Clone {
    fn from_f64(x: f64) -> Self;
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn sin_tau(self) -> Self;
    fn cos_tau(self) -> Self;
    fn sin_cos_tau(self) -> (Self, Self) {
        (self.clone().sin_tau(), self.cos_tau())
    }
    fn atan2(self, x: Self) -> Self;
}

impl Real for f64 {
    fn from_f64(x: f64) -> Self {
        x
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn cbrt(self) -> Self {
        self.cbrt()
    }

    fn sin_tau(self) -> Self {
        (self * std::f64::consts::TAU).sin()
    }

    fn cos_tau(self) -> Self {
        (self * std::f64::consts::TAU).cos()
    }

    fn sin_cos_tau(self) -> (Self, Self) {
        (self * std::f64::consts::TAU).sin_cos()
    }

    fn atan2(self, x: Self) -> Self {
        self.atan2(x)
    }
}

impl Real for f32 {
    fn from_f64(x: f64) -> Self {
        x as f32
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn cbrt(self) -> Self {
        self.cbrt()
    }

    fn atan2(self, x: Self) -> Self {
        self.atan2(x)
    }

    fn sin_tau(self) -> Self {
        (self * std::f32::consts::TAU).sin()
    }

    fn cos_tau(self) -> Self {
        (self * std::f32::consts::TAU).cos()
    }

    fn sin_cos_tau(self) -> (Self, Self) {
        (self * std::f32::consts::TAU).sin_cos()
    }
}
