use crate::algebra::abstract_::group::{Group, Monoid};
use crate::algebra::abstract_::{
    Additive, Curve, DifferentiableCurve, Functor, Ring, Semigroup, VectorSpace,
};

use super::cubic::Cubic;
use super::linear::Linear;
use super::quadratic::Quadratic;

/// Wrapper that takes a polynomial over a [`VectorSpace`], and turns it into a polynomial over a [`Ring`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OverRing<T> {
    pub over_vector_space: T,
}

impl<T: Semigroup> Semigroup for OverRing<T> {
    fn compose(self, rhs: Self) -> Self {
        OverRing {
            over_vector_space: self.over_vector_space.compose(rhs.over_vector_space),
        }
    }
}

impl<T: Ring> Curve for OverRing<Linear<T>> {
    type Domain = T;
    type Codomain = T;
    fn evaluate(self, x: T) -> T {
        self.over_vector_space.evaluate_ring(x)
    }
}

impl<T: Ring> Curve for OverRing<Quadratic<T>> {
    type Domain = T;
    type Codomain = T;
    fn evaluate(self, x: T) -> T {
        self.over_vector_space.evaluate_ring(x)
    }
}

impl<T: Ring> Curve for OverRing<Cubic<T>> {
    type Domain = T;
    type Codomain = T;
    fn evaluate(self, x: T) -> T {
        self.over_vector_space.evaluate_ring(x)
    }
}

impl<T: Ring> DifferentiableCurve for OverRing<Cubic<T>> {
    type Derivative = OverRing<Quadratic<T>>;
    fn derivative(self) -> Self::Derivative {
        OverRing {
            over_vector_space: self.over_vector_space.derivative_ring(),
        }
    }
}

impl<T: Ring> DifferentiableCurve for OverRing<Quadratic<T>> {
    type Derivative = OverRing<Linear<T>>;
    fn derivative(self) -> Self::Derivative {
        OverRing {
            over_vector_space: self.over_vector_space.derivative_ring(),
        }
    }
}

impl<T: Ring> DifferentiableCurve for OverRing<Linear<T>> {
    // The derivative of an affine map is the constant `c1`.
    type Derivative = T;
    fn derivative(self) -> Self::Derivative {
        self.over_vector_space.derivative_ring()
    }
}

impl<T: Additive> Additive for OverRing<T> {
    fn plus(self, rhs: Self) -> Self {
        OverRing {
            over_vector_space: self.over_vector_space.plus(rhs.over_vector_space),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        OverRing {
            over_vector_space: self.over_vector_space.minus(rhs.over_vector_space),
        }
    }

    fn zero() -> Self {
        OverRing {
            over_vector_space: T::zero(),
        }
    }

    fn negate(self) -> Self {
        OverRing {
            over_vector_space: self.over_vector_space.negate(),
        }
    }
}

impl<T: Monoid> Monoid for OverRing<T> {
    fn identity() -> Self {
        OverRing {
            over_vector_space: T::identity(),
        }
    }
}

impl<T: Group> Group for OverRing<T> {
    fn inverse(self) -> Self {
        OverRing {
            over_vector_space: self.over_vector_space.inverse(),
        }
    }
}

impl<T: Functor> Functor for OverRing<T> {
    type Param = T::Param;
    type Output<B> = OverRing<T::Output<B>>;
    fn map<B, F: FnMut(T::Param) -> B>(self, f: F) -> Self::Output<B> {
        OverRing {
            over_vector_space: self.over_vector_space.map(f),
        }
    }
}

impl<T: VectorSpace> VectorSpace for OverRing<T> {
    type Scalar = T::Scalar;
    fn scale(self, c: Self::Scalar) -> Self {
        OverRing {
            over_vector_space: self.over_vector_space.scale(c),
        }
    }
}
