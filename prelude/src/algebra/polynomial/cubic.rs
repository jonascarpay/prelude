use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve, Ring, VectorSpace,
};
use super::linear::Linear;
use super::quadratic::Quadratic;

/// A degree 3 univariate polynomial, i.e. of the form `c3 * x^3 + c2 * x^2 + c1 * x + c0`
#[derive(Clone, Copy, Debug)]
pub struct Cubic<T> {
    pub c0: T,
    pub c1: T,
    pub c2: T,
    pub c3: T,
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

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero() && self.c2.is_zero() && self.c3.is_zero()
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

impl<T: Ring + Copy> Curve for Cubic<T> {
    type Domain = T;
    type Codomain = T;
    // This seems wrong
    fn evaluate(self, x: T) -> T {
        self.c0
            .mult(x.cubed())
            .plus(self.c1)
            .mult(x.squared())
            .plus(self.c2)
            .mult(x)
            .plus(self.c3)
    }
}

impl<T: Ring + Copy> DifferentiableCurve for Cubic<T> {
    type Derivative = Quadratic<T>;
    fn derivative(self) -> Self::Derivative {
        Quadratic {
            c0: self.c1,
            c1: self.c2.imult(2),
            c2: self.c3.imult(3),
        }
    }
}

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

pub fn bezier3<T>(p0: T, p1: T, p2: T, p3: T) -> Cubic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Cubic {
        c0: p0.clone(),
        c1: p0.iscaled(-3).plus(p1.iscaled(3)),
        c2: p0.iscaled(3).plus(p1.iscaled(-6)).plus(p2.iscaled(3)),
        c3: p0.negate().plus(p1.iscale(3)).plus(p2.iscale(-2).plus(p3)),
    }
}
