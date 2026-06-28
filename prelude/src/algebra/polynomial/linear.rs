use std::ops::Range;

use crate::algebra::{
    abstract_::{
        group::{Group, Monoid, Semigroup},
        Functor,
    },
    v2, V2,
};

use super::super::abstract_::{
    field::Field, impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve,
    Ring, VectorSpace,
};
use super::over_ring::OverRing;

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

/*
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
*/

pub fn remap<T: Field>(from: Range<T>, to: Range<T>) -> Linear<T> {
    let xa = from.start;
    let xb = from.end;
    let ya = to.start;
    let yb = to.end;
    let dy = yb.minus(ya.clone());
    let dx = xb.minus(xa.clone());
    let c = dy.div(dx);
    // equivalently: lerp(ya, yb).compose(lerp(xa, xb).inverse())
    Linear {
        c1: c.clone(),
        c0: ya.minus(xa.mult(c)),
    }
}

// This is really just zipping, might be worth generalizing.
pub fn remap2<T: Field>(from: Range<V2<T>>, to: Range<V2<T>>) -> V2<Linear<T>> {
    v2(
        remap(from.start.x..from.end.x, to.start.x..to.end.x),
        remap(from.start.y..from.end.y, to.start.y..to.end.y),
    )
}

impl<T: Ring> Semigroup for Linear<T> {
    fn compose(self, rhs: Self) -> Self {
        let Linear { c1: a, c0: b } = self;
        let Linear { c1: c, c0: d } = rhs;
        Linear {
            c1: a.clone().mult(c),
            c0: a.mult(d).plus(b),
        }
    }
}

impl<T: Ring> Monoid for Linear<T> {
    fn identity() -> Self {
        Linear {
            c1: T::one(),
            c0: T::zero(),
        }
    }
}

impl<T: Field> Group for Linear<T> {
    fn inverse(self) -> Self {
        let inv = self.c1.recip();
        Linear {
            c1: inv.clone(),
            c0: self.c0.negate().mult(inv),
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

impl<T: Ring> Linear<T> {
    pub fn evaluate_ring(self, x: T) -> T {
        self.c1.mult(x).plus(self.c0)
    }

    pub fn derivative_ring(self) -> T {
        self.c1
    }

    pub fn over_ring(self) -> OverRing<Self> {
        OverRing {
            over_vector_space: self,
        }
    }
}

impl<T: VectorSpace> Linear<T> {
    pub fn evaluate_vector_space(self, x: T::Scalar) -> T {
        self.c1.scale(x).plus(self.c0)
    }

    pub fn derivative_vector_space(self) -> T {
        self.c1
    }
}

impl<T: VectorSpace> Curve for Linear<T> {
    type Domain = T::Scalar;
    type Codomain = T;
    fn evaluate(self, x: T::Scalar) -> T {
        self.evaluate_vector_space(x)
    }
}

impl<T: VectorSpace> DifferentiableCurve for Linear<T> {
    type Derivative = T;
    fn derivative(self) -> Self::Derivative {
        self.derivative_vector_space()
    }
}

impl<T: Additive> From<T> for Linear<T> {
    fn from(c0: T) -> Self {
        Linear { c0, ..Self::zero() }
    }
}

impl<T> Functor for Linear<T> {
    type Param = T;
    type Output<B> = Linear<B>;
    fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> Self::Output<B> {
        Linear {
            c0: f(self.c0),
            c1: f(self.c1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::numeric::rational::proptest_impls::gen_ratio;

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn lerp_0_is_start(start: i32, end: i32) {
            prop_assert_eq!(lerp(start, end).evaluate(0), start);
        }

        #[test]
        fn lerp_1_is_end(start: i32, end: i32) {
            prop_assert_eq!(lerp(start, end).evaluate(1), end);
        }

        #[test]
        fn remap_remaps_start(
            domain_start in gen_ratio(),
            domain_end in gen_ratio(),
            range_start in gen_ratio(),
            range_end in gen_ratio()
        ) {
            prop_assume!(domain_start != domain_end);
            prop_assert_eq!(
                remap(domain_start..domain_end, range_start..range_end).evaluate_ring(domain_start),
                range_start
            )
        }

        #[test]
        fn remap_remaps_end(
            domain_start in gen_ratio(),
            domain_end in gen_ratio(),
            range_start in gen_ratio(),
            range_end in gen_ratio()
        ) {
            prop_assume!(domain_start != domain_end);
            prop_assert_eq!(
                remap(domain_start..domain_end, range_start..range_end).evaluate_ring(domain_end),
                range_end
            )
        }
    }
}
