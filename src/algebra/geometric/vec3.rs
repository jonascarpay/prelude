use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Ring, VectorSpace,
};

pub struct V3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> V3<T> {
    pub fn new(x: T, y: T, z: T) -> V3<T> {
        V3 { x, y, z }
    }
}

impl<T: Additive> Additive for V3<T> {
    fn plus(self, rhs: Self) -> Self {
        V3 {
            x: self.x.plus(rhs.x),
            y: self.y.plus(rhs.y),
            z: self.z.plus(rhs.z),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        V3 {
            x: self.x.minus(rhs.x),
            y: self.y.minus(rhs.y),
            z: self.z.minus(rhs.z),
        }
    }

    fn zero() -> Self {
        V3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn negate(self) -> Self {
        V3 {
            x: self.x.negate(),
            y: self.y.negate(),
            z: self.z.negate(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T: Ring + Copy> VectorSpace for V3<T> {
    type Over = T;

    fn scale(self, c: Self::Over) -> Self {
        V3 {
            x: self.x.mult(c),
            y: self.y.mult(c),
            z: self.z.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] V3<T>);
impl_vector_space_ops!([T: Ring + Copy] V3<T>);
