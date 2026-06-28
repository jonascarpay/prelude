use crate::algebra::abstract_::Functor;

use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve, Ring,
    VectorSpace,
};
use super::linear::Linear;
use super::over_ring::OverRing;

/// A degree 2 univariate polynomial, i.e. of the form `c2 * x^2 + c1 * x + c0`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quadratic<T> {
    pub c0: T,
    pub c1: T,
    pub c2: T,
}

impl<T> Quadratic<T> {
    pub fn from_roots(a: T, r1: T, r2: T) -> Self
    where
        T: Ring + Clone,
    {
        // a(x - r1)(x - r2)
        // a(x^2 - r1 x - r2 x + r1 r2)
        // a x^2 + a (- r1 - r2) x + a r1 r2
        Quadratic {
            c0: a.clone().mult(r1.clone()).mult(r2.clone()),
            c1: a.clone().mult(r1.plus(r2).negate()),
            c2: a,
        }
    }
}

impl<T: Additive> Additive for Quadratic<T> {
    fn plus(self, rhs: Self) -> Self {
        Quadratic {
            c0: self.c0.plus(rhs.c0),
            c1: self.c1.plus(rhs.c1),
            c2: self.c2.plus(rhs.c2),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Quadratic {
            c0: self.c0.minus(rhs.c0),
            c1: self.c1.minus(rhs.c1),
            c2: self.c2.minus(rhs.c2),
        }
    }

    fn zero() -> Self {
        Quadratic {
            c0: T::zero(),
            c1: T::zero(),
            c2: T::zero(),
        }
    }

    fn negate(self) -> Self {
        Quadratic {
            c0: self.c0.negate(),
            c1: self.c1.negate(),
            c2: self.c2.negate(),
        }
    }
}

impl<T: Ring + Copy> VectorSpace for Quadratic<T> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Quadratic {
            c0: self.c0.mult(c),
            c1: self.c1.mult(c),
            c2: self.c2.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] Quadratic<T>);
impl_vector_space_ops!([T: Ring + Copy] Quadratic<T>);

impl<T: Ring> Quadratic<T> {
    pub fn evaluate_ring(self, x: T) -> T {
        self.c0
            .plus(self.c1.mult(x.clone()))
            .plus(self.c2.mult(x.clone().squared()))
    }

    pub fn derivative_ring(self) -> Linear<T> {
        Linear {
            c0: self.c1,
            c1: self.c2.imult(2),
        }
    }

    pub fn over_ring(self) -> OverRing<Self> {
        OverRing {
            over_vector_space: self,
        }
    }
}

impl<T: VectorSpace> Quadratic<T> {
    pub fn evaluate_vector_space(self, x: T::Scalar) -> T {
        self.c0
            .plus(self.c1.scale(x.clone()))
            .plus(self.c2.scale(x.clone().squared()))
    }

    pub fn derivative_vector_space(self) -> Linear<T> {
        Linear {
            c0: self.c1,
            c1: self.c2.iscale(2),
        }
    }
}

impl<T: VectorSpace> Curve for Quadratic<T> {
    type Domain = T::Scalar;
    type Codomain = T;
    fn evaluate(self, x: T::Scalar) -> T {
        self.evaluate_vector_space(x)
    }
}

impl<T: VectorSpace> DifferentiableCurve for Quadratic<T> {
    type Derivative = Linear<T>;
    fn derivative(self) -> Self::Derivative {
        self.derivative_vector_space()
    }
}

impl<T: Additive> From<T> for Quadratic<T> {
    fn from(c0: T) -> Self {
        Quadratic { c0, ..Self::zero() }
    }
}

impl<T: Additive> From<Linear<T>> for Quadratic<T> {
    fn from(l: Linear<T>) -> Self {
        Quadratic {
            c0: l.c0,
            c1: l.c1,
            ..Self::zero()
        }
    }
}

impl<T> Functor for Quadratic<T> {
    type Param = T;
    type Output<B> = Quadratic<B>;
    fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> Quadratic<B> {
        Quadratic {
            c0: f(self.c0),
            c1: f(self.c1),
            c2: f(self.c2),
        }
    }
}

pub fn bezier2<T>(p0: T, p1: T, p2: T) -> Quadratic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Quadratic {
        c0: p0.clone(),
        c1: p0.iscaled(-2).plus(p1.iscaled(1)),
        c2: p0.plus(p1.iscale(-2)).plus(p2),
    }
}
