use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve, Ring, VectorSpace,
};

/// A degree 1 univariate polynomial, i.e. of the form `c1 * x + c0`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Linear<T> {
    pub c1: T,
    pub c0: T,
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
