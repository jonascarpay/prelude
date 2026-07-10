use std::ops::Range;

use crate::algebra::abstract_::{
    group::{Group, Monoid, Semigroup},
    Functor,
};

use super::super::abstract_::{
    field::Field, impl_additive_ops, impl_vector_space_ops, Additive, Ring, VectorSpace,
};

/// An affine map, i.e. a combination of scaling and translating.
///
/// Functionally identical to a degree 1 univariate polynomial, but unlike a general polynomial,
/// this forms a group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Affine<T> {
    pub c1: T,
    pub c0: T,
}

pub fn lerp<T: Additive>(zero: T, one: T) -> Affine<T> {
    Affine {
        c1: one.minus(zero.clone()),
        c0: zero,
    }
}

pub fn remap<T: Field>(from: Range<T>, to: Range<T>) -> Affine<T> {
    let xa = from.start;
    let xb = from.end;
    let ya = to.start;
    let yb = to.end;
    let dy = yb.minus(ya.clone());
    let dx = xb.minus(xa.clone());
    let c = dy.div(dx);
    // equivalently: lerp(ya, yb).compose(lerp(xa, xb).inverse())
    Affine {
        c1: c.clone(),
        c0: ya.minus(xa.mult(c)),
    }
}

impl<T> Affine<T> {
    pub fn evaluate_ring(self, x: T) -> T
    where
        T: Ring,
    {
        self.c1.mult(x).plus(self.c0)
    }

    pub fn derivative(self) -> T {
        self.c1
    }

    pub fn evaluate_vector_space(self, x: T::Scalar) -> T
    where
        T: VectorSpace,
    {
        self.c1.scale(x).plus(self.c0)
    }
}

impl<T: Ring> Semigroup for Affine<T> {
    fn compose(self, rhs: Self) -> Self {
        let Affine { c1: a, c0: b } = self;
        let Affine { c1: c, c0: d } = rhs;
        Affine {
            c1: a.clone().mult(c),
            c0: a.mult(d).plus(b),
        }
    }
}

impl<T: Ring> Monoid for Affine<T> {
    fn identity() -> Self {
        Affine {
            c1: T::ONE,
            c0: T::ZERO,
        }
    }
}

impl<T: Field> Group for Affine<T> {
    fn inverse(self) -> Self {
        let inv = self.c1.recip();
        Affine {
            c1: inv.clone(),
            c0: self.c0.negate().mult(inv),
        }
    }
}

impl<T: Additive> Additive for Affine<T> {
    const ZERO: Self = Affine {
        c1: T::ZERO,
        c0: T::ZERO,
    };

    fn plus(self, rhs: Self) -> Self {
        Affine {
            c1: self.c1.plus(rhs.c1),
            c0: self.c0.plus(rhs.c0),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Affine {
            c1: self.c1.minus(rhs.c1),
            c0: self.c0.minus(rhs.c0),
        }
    }

    fn negate(self) -> Self {
        Affine {
            c1: self.c1.negate(),
            c0: self.c0.negate(),
        }
    }
}

impl<T: Ring + Copy> VectorSpace for Affine<T> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Affine {
            c1: self.c1.mult(c),
            c0: self.c0.mult(c),
        }
    }
}

impl<T: Additive> From<T> for Affine<T> {
    fn from(c0: T) -> Self {
        Affine { c0, ..Self::ZERO }
    }
}

impl<T> Functor for Affine<T> {
    type Param = T;
    type Output<B> = Affine<B>;
    fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> Self::Output<B> {
        Affine {
            c0: f(self.c0),
            c1: f(self.c1),
        }
    }
}

impl_additive_ops!([T: Additive] Affine<T>);
impl_vector_space_ops!([T: Ring + Copy] Affine<T>);

#[cfg(test)]
mod tests {
    use crate::algebra::numeric::rational::proptest_impls::gen_ratio;

    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn lerp_0_is_start(start: i32, end: i32) {
            prop_assert_eq!(lerp(start, end).evaluate_ring(0), start);
        }

        #[test]
        fn lerp_1_is_end(start: i32, end: i32) {
            prop_assert_eq!(lerp(start, end).evaluate_ring(1), end);
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
