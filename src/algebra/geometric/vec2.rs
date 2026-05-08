use super::super::abstract_::{Additive, Ring, VectorSpace};

pub struct V2<T> {
    pub x: T,
    pub y: T,
}

impl<T> V2<T> {
    pub fn new(x: T, y: T) -> V2<T> {
        V2 { x, y }
    }
}

impl<T: Additive> Additive for V2<T> {
    fn plus(self, rhs: Self) -> Self {
        V2 {
            x: self.x.plus(rhs.x),
            y: self.y.plus(rhs.y),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        V2 {
            x: self.x.minus(rhs.x),
            y: self.y.minus(rhs.y),
        }
    }

    fn zero() -> Self {
        V2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn negate(self) -> Self {
        V2 {
            x: self.x.negate(),
            y: self.y.negate(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Ring + Copy> VectorSpace for V2<T> {
    type Over = T;

    fn scale(self, c: Self::Over) -> Self {
        V2 {
            x: self.x.mult(c),
            y: self.y.mult(c),
        }
    }
}
