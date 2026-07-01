use crate::algebra::abstract_::field::Field;

/// A bit of a grab bag of functions that can't live in weaker traits.
pub trait Real: Field + Clone {
    fn from_f64(x: f64) -> Self;
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sin_cos(self) -> (Self, Self) {
        (self.clone().sin(), self.cos())
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

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn sin_cos(self) -> (Self, Self) {
        f64::sin_cos(self)
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

    fn sin(self) -> Self {
        self.sin()
    }

    fn cos(self) -> Self {
        self.cos()
    }

    fn sin_cos(self) -> (Self, Self) {
        f32::sin_cos(self)
    }

    fn atan2(self, x: Self) -> Self {
        self.atan2(x)
    }
}
