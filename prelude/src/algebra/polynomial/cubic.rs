use crate::algebra::abstract_::ring::one;
use crate::algebra::abstract_::Functor;

use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Ring, VectorSpace,
};
use super::linear::Linear;
use super::quadratic::Quadratic;

/// A degree 3 univariate polynomial, i.e. of the form `c3 * x^3 + c2 * x^2 + c1 * x + c0`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cubic<T> {
    pub c0: T,
    pub c1: T,
    pub c2: T,
    pub c3: T,
}

impl<T> Cubic<T> {
    /// Construct a cubic from the factored from `(x - r1)(x - r2)(x - r3)`
    pub fn from_roots(r1: T, r2: T, r3: T) -> Self
    where
        T: Copy + Ring, // TODO Clone
    {
        // (x - r1)(x - r2)(x - r3)
        // (x - r1)(x^2 - r2 x - r3 x + r2 r3)
        // x^3 - r2 x^2 - r3 x^2 + r2 r3 x - r1 x^2 + r1 r2 x + r1 r3 x - r1 r2 r3
        //
        // x^3
        // (- r1 - r2 - r3) x^2
        // (r2 r3 + r1 r2 + r1 r3) x
        // (- r1 r2 r3)
        Cubic {
            c0: r1.mult(r2).mult(r3).negate(),
            c1: (r1.mult(r2)).plus(r2.mult(r3)).plus(r1.mult(r3)),
            c2: r1.negate().minus(r2).minus(r3),
            c3: one(),
        }
    }

    pub fn evaluate_ring(self, x: T) -> T
    where
        T: Ring,
    {
        let Cubic { c0, c1, c2, c3 } = self;
        c0.plus(x.clone().mult(c1.plus(x.clone().mult(c2.plus(x.mult(c3))))))
    }

    pub fn derivative_ring(self) -> Quadratic<T>
    where
        T: Ring,
    {
        Quadratic {
            c0: self.c1,
            c1: self.c2.imult(2),
            c2: self.c3.imult(3),
        }
    }

    pub fn evaluate_vector_space(self, x: T::Scalar) -> T
    where
        T: VectorSpace,
    {
        self.c0
            .plus(self.c1.scale(x.clone()))
            .plus(self.c2.scale(x.clone().squared()))
            .plus(self.c3.scale(x.cubed()))
    }

    pub fn derivative_vector_space(self) -> Quadratic<T>
    where
        T: VectorSpace,
    {
        Quadratic {
            c0: self.c1,
            c1: self.c2.iscale(2),
            c2: self.c3.iscale(3),
        }
    }
}

impl<T: Additive> Additive for Cubic<T> {
    fn plus(self, rhs: Self) -> Self {
        Cubic {
            c0: self.c0.plus(rhs.c0),
            c1: self.c1.plus(rhs.c1),
            c2: self.c2.plus(rhs.c2),
            c3: self.c3.plus(rhs.c3),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Cubic {
            c0: self.c0.minus(rhs.c0),
            c1: self.c1.minus(rhs.c1),
            c2: self.c2.minus(rhs.c2),
            c3: self.c3.minus(rhs.c3),
        }
    }

    fn zero() -> Self {
        Cubic {
            c0: T::zero(),
            c1: T::zero(),
            c2: T::zero(),
            c3: T::zero(),
        }
    }

    fn negate(self) -> Self {
        Cubic {
            c0: self.c0.negate(),
            c1: self.c1.negate(),
            c2: self.c2.negate(),
            c3: self.c3.negate(),
        }
    }
}

impl<T: Ring + Copy> VectorSpace for Cubic<T> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Cubic {
            c0: self.c0.mult(c),
            c1: self.c1.mult(c),
            c2: self.c2.mult(c),
            c3: self.c3.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] Cubic<T>);
impl_vector_space_ops!([T: Ring + Copy] Cubic<T>);

impl<T: Additive> From<T> for Cubic<T> {
    fn from(c0: T) -> Self {
        Cubic { c0, ..Self::zero() }
    }
}

impl<T: Additive> From<Linear<T>> for Cubic<T> {
    fn from(l: Linear<T>) -> Self {
        Cubic {
            c0: l.c0,
            c1: l.c1,
            ..Self::zero()
        }
    }
}

impl<T: Additive> From<Quadratic<T>> for Cubic<T> {
    fn from(q: Quadratic<T>) -> Self {
        Cubic {
            c0: q.c0,
            c1: q.c1,
            c2: q.c2,
            ..Self::zero()
        }
    }
}

impl<T> Functor for Cubic<T> {
    type Param = T;
    type Output<B> = Cubic<B>;
    fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> Cubic<B> {
        Cubic {
            c0: f(self.c0),
            c1: f(self.c1),
            c2: f(self.c2),
            c3: f(self.c3),
        }
    }
}

#[inline(always)]
pub fn unit_hermite3<T>(p_start: T, v_start: T, p_end: T, v_end: T) -> Cubic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Cubic {
        c0: p_start.clone(),
        c1: v_start.clone(),
        c2: (p_start.iscaled(-3))
            .plus(p_end.iscaled(3))
            .plus(v_start.iscaled(-2))
            .plus(v_end.negated()),
        c3: (p_start.iscale(2))
            .plus(p_end.iscale(-2))
            .plus(v_start)
            .plus(v_end),
    }
}

/// A cubic Bezier polynomial, given a start point, two control points, and an end point.
// TODO document unit domain
pub fn bezier3<T>(p0: T, p1: T, p2: T, p3: T) -> Cubic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Cubic {
        c0: p0.clone(),
        c1: p0.iscaled(-3).plus(p1.iscaled(3)),
        c2: p0.iscaled(3).plus(p1.iscaled(-6)).plus(p2.iscaled(3)),
        c3: p0.negate().plus(p1.iscale(3)).plus(p2.iscale(-3).plus(p3)),
    }
}
