use crate::{algebra::abstract_::field::Field, impl_ring_ops};

use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, InnerProductSpace, Ring, VectorSpace,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A complex number of the form a + bi.
/// We tie it into the geometric algebra by interpreting it as the even subalgebra of Cl(2,0,0)
pub struct Complex<T> {
    pub s: T,
    pub xy: T,
}

impl<T: Ring + Clone> Complex<T> {
    pub const ONE: Self = Complex {
        s: T::ONE,
        xy: T::ZERO,
    };
    pub const XY: Self = Complex {
        s: T::ZERO,
        xy: T::ONE,
    };
    pub const I: Self = Self::XY;
    pub const BASIS: [Self; 2] = [Self::ONE, Self::XY];

    // This is the same as reverse, and might be moved to a type class at some point
    pub fn conjugate(self) -> Self {
        Complex {
            s: self.s,
            xy: self.xy.negate(),
        }
    }
    pub fn magnitude(self) -> T {
        self.s.squared().plus(self.xy.squared())
    }
}

impl Complex<f64> {
    pub fn argument(self) -> f64 {
        self.xy.atan2(self.s)
    }
}

impl<T: Additive> Additive for Complex<T> {
    const ZERO: Self = Self::ZERO;
    fn minus(self, rhs: Self) -> Self {
        self.plus(rhs.negate())
    }

    fn plus(self, rhs: Self) -> Self {
        Complex {
            s: self.s.plus(rhs.s),
            xy: self.xy.plus(rhs.xy),
        }
    }

    fn negate(self) -> Self {
        Complex {
            s: self.s.negate(),
            xy: self.xy.negate(),
        }
    }
}

impl<T: Ring + Clone> VectorSpace for Complex<T> {
    type Scalar = T;

    fn scale(self, c: T) -> Self {
        Complex {
            s: self.s.mult(c.clone()),
            xy: self.xy.mult(c),
        }
    }
}

impl<T: Ring + Copy> InnerProductSpace for Complex<T> {
    // TODO I hate this name
    fn quadrance(self) -> Self::Scalar {
        // Q(a + bi)
        // (a + bi)(a - bi)
        // aa - abi + bia + bb
        // aa + bb
        self.s.squared().plus(self.xy.squared())
    }

    fn inner(self, rhs: Self) -> Self::Scalar {
        // 1/2 (Q(u+v) - Q(u) - Q(v))
        // 1/2 (Q((a+c) + (b+d)i) - Q(a + ci) - Q(b + di))
        // 1/2 (aa + cc + 2ac + bb + dd + 2bd - aa - cc - bb - dd)
        // 1/2 (2ac + 2bd)
        // ac + bd
        self.s.mult(rhs.s).plus(self.xy.plus(rhs.xy))
    }
}

impl<T: Ring> From<T> for Complex<T> {
    fn from(value: T) -> Self {
        Complex {
            s: value,
            xy: T::ZERO,
        }
    }
}

impl<T: Ring + Clone> Ring for Complex<T> {
    const ONE: Self = Self::ONE;

    fn mult(self, rhs: Self) -> Self {
        // (a + bi) (c + di)
        // ac + adi + bci - bd
        // (ac - bd)1 + (ad + bc)i
        let Complex { s: a, xy: b } = self;
        let Complex { s: c, xy: d } = rhs;
        Complex {
            s: a.clone().mult(c.clone()).minus(b.clone().mult(d.clone())),
            xy: a.mult(d).plus(b.mult(c)),
        }
    }

    fn from_integer(i: isize) -> Self {
        Complex {
            s: T::from_integer(i),
            ..Self::ZERO
        }
    }
}

impl<T: Field> Field for Complex<T> {
    fn recip(self) -> Self {
        let Complex { s: a, xy: b } = self;
        let q = a.clone().squared().plus(b.clone().squared());

        Complex {
            s: a.div(q.clone()),
            xy: b.div(q).negate(),
        }
    }

    fn div(self, rhs: Self) -> Self {
        let Complex { s: a, xy: b } = self;
        let Complex { s: c, xy: d } = rhs;
        let q = c.clone().squared().plus(d.clone().squared());

        Complex {
            s: a.clone()
                .mult(c.clone())
                .plus(b.clone().mult(d.clone()))
                .div(q.clone()),
            xy: b.mult(c).minus(a.mult(d)).div(q),
        }
    }

    fn checked_div(self, rhs: Self) -> Option<Self> {
        let Complex { s: a, xy: b } = self;
        let Complex { s: c, xy: d } = rhs;
        let q = c.clone().squared().plus(d.clone().squared());

        Some(Complex {
            s: a.clone()
                .mult(c.clone())
                .plus(b.clone().mult(d.clone()))
                .checked_div(q.clone())?,
            xy: b.mult(c).minus(a.mult(d)).checked_div(q)?,
        })
    }
}

impl_additive_ops!([T: Additive] Complex<T>);
impl_vector_space_ops!([T: Ring + Copy] Complex<T>);
impl_ring_ops!([T: Ring + Copy] Complex<T>);
