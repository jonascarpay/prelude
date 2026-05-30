use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve, Ring, VectorSpace,
};
use super::linear::Linear;

/// A degree 2 univariate polynomial, i.e. of the form `c2 * x^2 + c1 * x + c0`
#[derive(Clone, Copy, Debug)]
pub struct Quadratic<T> {
    pub c0: T,
    pub c1: T,
    pub c2: T,
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

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero() && self.c2.is_zero()
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

impl<T: VectorSpace> Curve for Quadratic<T> {
    type Domain = T::Scalar;
    type Codomain = T;
    fn evaluate(self, x: T::Scalar) -> T {
        self.c0
            .scale(x.clone())
            .plus(self.c1.scale(x.clone()))
            .plus(self.c2.scale(x.clone().squared()))
    }
}

impl<T: Ring + Copy> DifferentiableCurve for Quadratic<T> {
    type Derivative = Linear<T>;
    fn derivative(self) -> Self::Derivative {
        Linear {
            c0: self.c1,
            c1: self.c2.mult(T::from_integer(2)),
        }
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
