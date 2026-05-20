use crate::algebra::abstract_::vector_space::linear_combination;

use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve, Ring,
    VectorSpace,
};
use super::linear::Linear;

/// A degree 2 univariate polynomial, i.e. of the form `c2 * x^2 + c1 * x + c0`
#[derive(Clone, Copy, Debug)]
pub struct Quadratic<T> {
    pub c2: T,
    pub c1: T,
    pub c0: T,
}

impl<T> Quadratic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    pub fn from_int_matrix<const N: usize>(mat: [[isize; N]; 3], pts: [T; N]) -> Self {
        let [r0, r1, r2]: [[T::Scalar; N]; 3] = mat.map(|r| r.map(Ring::from_integer));
        Quadratic {
            c0: linear_combination(r0, pts.clone()),
            c1: linear_combination(r1, pts.clone()),
            c2: linear_combination(r2, pts),
        }
    }
}

impl<T: Additive> Additive for Quadratic<T> {
    fn plus(self, rhs: Self) -> Self {
        Quadratic {
            c2: self.c2.plus(rhs.c2),
            c1: self.c1.plus(rhs.c1),
            c0: self.c0.plus(rhs.c0),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Quadratic {
            c2: self.c2.minus(rhs.c2),
            c1: self.c1.minus(rhs.c1),
            c0: self.c0.minus(rhs.c0),
        }
    }

    fn zero() -> Self {
        Quadratic {
            c2: T::zero(),
            c1: T::zero(),
            c0: T::zero(),
        }
    }

    fn negate(self) -> Self {
        Quadratic {
            c2: self.c2.negate(),
            c1: self.c1.negate(),
            c0: self.c0.negate(),
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
            c2: self.c2.mult(c),
            c1: self.c1.mult(c),
            c0: self.c0.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] Quadratic<T>);
impl_vector_space_ops!([T: Ring + Copy] Quadratic<T>);

impl<T: Ring + Copy> Curve for Quadratic<T> {
    type Domain = T;
    type Range = T;
    fn evaluate(self, x: T) -> T {
        self.c2.mult(x).plus(self.c1).mult(x).plus(self.c0)
    }
}

impl<T: Ring + Copy> DifferentiableCurve for Quadratic<T> {
    type Derivative = Linear<T>;
    fn derivative(self) -> Self::Derivative {
        Linear {
            c1: self.c2.mult(T::from_integer(2)),
            c0: self.c1,
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
            c1: l.c1,
            c0: l.c0,
            ..Self::zero()
        }
    }
}

pub fn bezier2<T>(p0: T, p1: T, p2: T) -> Quadratic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Quadratic::from_int_matrix([[1, 0, 0], [-2, 2, 0], [1, -2, 1]], [p0, p1, p2])
}
