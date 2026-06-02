use std::ops::Range;

use super::super::abstract_::{
    field::Field, impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve, Ring, VectorSpace,
};

/// A degree 1 univariate polynomial, i.e. of the form `c1 * x + c0`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Linear<T> {
    pub c1: T,
    pub c0: T,
}

pub fn lerp<T: Additive>(zero: T, one: T) -> Linear<T> {
    Linear {
        c1: one.minus(zero.clone()),
        c0: zero,
    }
}

pub fn remap<T: Field>(from: Range<T>, to: Range<T>) -> Linear<T> {
    let xa = from.start;
    let xb = from.end;
    let ya = to.start;
    let yb = to.end;
    // equivalently: lerp(ya, yb).compose(lerp(xa, xb).inverse())
    let inv = (xb.minus(xa.clone())).recip();
    Linear {
        c1: yb.minus(ya.clone()).mult(inv),
        c0: ya.minus(xa),
    }
}

impl<T: Field> Linear<T> {
    pub fn inverse(self) -> Self {
        let inv = self.c1.recip();
        Linear {
            c1: inv.clone(),
            c0: self.c0.negate().mult(inv),
        }
    }
    pub fn identity() -> Self {
        Linear {
            c1: T::one(),
            c0: T::zero(),
        }
    }
    // Forms a group!
    pub fn compose(self, rhs: Self) -> Self {
        let Linear { c1: a, c0: b } = self;
        let Linear { c1: c, c0: d } = rhs;
        Linear {
            c1: a.clone().mult(c),
            c0: a.mult(d).plus(b),
        }
    }
}

impl<T: Additive> Additive for Linear<T> {
    fn plus(self, rhs: Self) -> Self {
        Linear {
            c1: self.c1.plus(rhs.c1),
            c0: self.c0.plus(rhs.c0),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Linear {
            c1: self.c1.minus(rhs.c1),
            c0: self.c0.minus(rhs.c0),
        }
    }

    fn zero() -> Self {
        Linear {
            c1: T::zero(),
            c0: T::zero(),
        }
    }

    fn negate(self) -> Self {
        Linear {
            c1: self.c1.negate(),
            c0: self.c0.negate(),
        }
    }
}

impl<T: Ring + Copy> VectorSpace for Linear<T> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Linear {
            c1: self.c1.mult(c),
            c0: self.c0.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] Linear<T>);
impl_vector_space_ops!([T: Ring + Copy] Linear<T>);

impl<T: Ring + Copy> Curve for Linear<T> {
    type Domain = T;
    type Codomain = T;
    fn evaluate(self, x: T) -> T {
        self.c1.mult(x).plus(self.c0)
    }
}

impl<T: Ring + Copy> DifferentiableCurve for Linear<T> {
    type Derivative = T;
    fn derivative(self) -> Self::Derivative {
        self.c1
    }
}

impl<T: Additive> From<T> for Linear<T> {
    fn from(c0: T) -> Self {
        Linear { c0, ..Self::zero() }
    }
}
