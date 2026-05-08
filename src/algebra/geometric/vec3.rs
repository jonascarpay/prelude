use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, InnerProductSpace, Ring, VectorSpace,
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

impl<T: Ring> V3<T> {
    pub fn x() -> Self {
        V3 {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }
    pub fn y() -> Self {
        V3 {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }
    pub fn z() -> Self {
        V3 {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }
    pub fn basis() -> [Self; 3] {
        [Self::x(), Self::y(), Self::z()]
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

impl<T: Ring + Clone> VectorSpace for V3<T> {
    type Over = T;

    fn scale(self, c: Self::Over) -> Self {
        V3 {
            x: self.x.mult(c.clone()),
            y: self.y.mult(c.clone()),
            z: self.z.mult(c),
        }
    }
}
impl<T: Ring + Clone> InnerProductSpace for V3<T> {
    fn quadrance(self) -> Self::Over {
        let V3 { x, y, z } = self;
        x.sq().plus(y.sq()).plus(z.sq())
    }

    fn inner(self, rhs: Self) -> Self::Over {
        let V3 {
            x: x0,
            y: y0,
            z: z0,
        } = self;
        let V3 {
            x: x1,
            y: y1,
            z: z1,
        } = rhs;
        x0.mult(x1).plus(y0.mult(y1)).plus(z0.mult(z1))
    }
}

impl_additive_ops!([T: Additive] V3<T>);
impl_vector_space_ops!([T: Ring + Copy] V3<T>);
