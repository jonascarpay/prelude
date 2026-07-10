use super::Point2D;
use crate::algebra::{one, zero, Additive, Ring, VectorSpace};

/// A line forming the solution to ax + by = c
#[derive(Copy, Clone, Debug)]
pub struct Line2D<T> {
    w: T,
    x: T,
    y: T,
}

impl<T: Ring> Line2D<T> {
    pub fn infinity() -> Self {
        Line2D { w: one(), ..zero() }
    }
    pub fn x_axis() -> Self {
        Line2D { x: one(), ..zero() }
    }
    pub fn y_axis() -> Self {
        Line2D { y: one(), ..zero() }
    }
    pub fn join(self, _rhs: Point2D<T>) -> f64 {
        // antiwedge
        todo!()
    }
    pub fn meet(self, _rhs: Self) -> Point2D<T> {
        // wedge
        todo!()
    }
}

impl<T: Clone> Additive for Line2D<T> {
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

impl<T: Ring> VectorSpace for Line2D<T> {
    type Scalar = T;
    fn scale(self, _rhs: T) -> Self {
        todo!()
    }
}
