use super::Line2D;
use crate::algebra::{one, zero, Additive, Ring, VectorSpace};

/// A point in 2D projective space
#[derive(Clone, Copy, Debug)]
pub struct Point2D<T> {
    xy: T,
    wx: T,
    wy: T,
}

impl<T: Ring> Point2D<T> {
    pub fn origin() -> Self {
        Point2D {
            xy: one(),
            ..zero()
        }
    }
    pub fn join(self, _rhs: Self) -> Line2D<T> {
        // antiwedge
        todo!()
    }
    pub fn meet(self, _rhs: Self) -> f64 {
        // wedge
        todo!()
    }

    pub fn translate(self, offset: Vector2D<T>) -> Self {}
}

impl<T: Clone> Additive for Point2D<T> {
    fn plus(self, _rhs: Self) -> Self {
        todo!()
    }
    fn zero() -> Self {
        todo!()
    }
    fn negate(self) -> Self {
        todo!()
    }
}

impl<T: Ring> VectorSpace for Point2D<T> {
    type Scalar = T;
    fn scale(self, _rhs: T) -> Self {
        todo!()
    }
}
