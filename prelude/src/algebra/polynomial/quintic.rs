use crate::algebra::abstract_::additive::iter_sum_reduce;
use crate::algebra::abstract_::field::Field;
use crate::algebra::abstract_::Functor;
use crate::algebra::polynomial::cubic::Cubic;

use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Ring, VectorSpace,
};
use super::linear::Linear;
use super::quadratic::Quadratic;

/// A degree 5 univariate polynomial, i.e. of the form `c5 x^5 + c4 x^4 + c3 x^3 + c2 x^2 + c1 x + c0`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quintic<T> {
    pub c0: T,
    pub c1: T,
    pub c2: T,
    pub c3: T,
    pub c4: T,
    pub c5: T,
}

impl<T> Quintic<T> {
    // TODO: from_roots, derivatives

    pub fn evaluate_ring(self, x: T) -> T
    where
        T: Ring,
    {
        let Quintic {
            c0,
            c1,
            c2,
            c3,
            c4,
            c5,
        } = self;
        c5.mult(x.clone())
            .plus(c4)
            .mult(x.clone())
            .plus(c3)
            .mult(x.clone())
            .plus(c2)
            .mult(x.clone())
            .plus(c1)
            .mult(x)
            .plus(c0)
    }

    pub fn evaluate_vector_space(self, x: T::Scalar) -> T
    where
        T: VectorSpace,
    {
        let x1 = x.clone();
        let x2 = x1.clone().mult(x.clone());
        let x3 = x2.clone().mult(x.clone());
        let x4 = x3.clone().mult(x.clone());
        let x5 = x4.clone().mult(x.clone());
        self.c0
            .plus(self.c1.scale(x1))
            .plus(self.c2.scale(x2))
            .plus(self.c3.scale(x3))
            .plus(self.c4.scale(x4))
            .plus(self.c5.scale(x5))
    }

    pub fn evaluate_vector_space_horner(self, x: T::Scalar) -> T
    where
        T: VectorSpace,
    {
        let Quintic {
            c0,
            c1,
            c2,
            c3,
            c4,
            c5,
        } = self;
        c5.scale(x.clone())
            .plus(c4)
            .scale(x.clone())
            .plus(c3)
            .scale(x.clone())
            .plus(c2)
            .scale(x.clone())
            .plus(c1)
            .scale(x)
            .plus(c0)
    }
}

impl<T: Additive> Additive for Quintic<T> {
    fn plus(self, rhs: Self) -> Self {
        Quintic {
            c0: self.c0.plus(rhs.c0),
            c1: self.c1.plus(rhs.c1),
            c2: self.c2.plus(rhs.c2),
            c3: self.c3.plus(rhs.c3),
            c4: self.c4.plus(rhs.c4),
            c5: self.c5.plus(rhs.c5),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Quintic {
            c0: self.c0.minus(rhs.c0),
            c1: self.c1.minus(rhs.c1),
            c2: self.c2.minus(rhs.c2),
            c3: self.c3.minus(rhs.c3),
            c4: self.c4.minus(rhs.c4),
            c5: self.c5.minus(rhs.c5),
        }
    }

    fn zero() -> Self {
        Quintic {
            c0: T::zero(),
            c1: T::zero(),
            c2: T::zero(),
            c3: T::zero(),
            c4: T::zero(),
            c5: T::zero(),
        }
    }

    fn negate(self) -> Self {
        Quintic {
            c0: self.c0.negate(),
            c1: self.c1.negate(),
            c2: self.c2.negate(),
            c3: self.c3.negate(),
            c4: self.c4.negate(),
            c5: self.c5.negate(),
        }
    }
}

impl<T: Ring + Copy> VectorSpace for Quintic<T> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Quintic {
            c0: self.c0.mult(c),
            c1: self.c1.mult(c),
            c2: self.c2.mult(c),
            c3: self.c3.mult(c),
            c4: self.c4.mult(c),
            c5: self.c5.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] Quintic<T>);
impl_vector_space_ops!([T: Ring + Copy] Quintic<T>);

impl<T: Additive> From<T> for Quintic<T> {
    fn from(c0: T) -> Self {
        Quintic { c0, ..Self::zero() }
    }
}

impl<T: Additive> From<Linear<T>> for Quintic<T> {
    fn from(l: Linear<T>) -> Self {
        Quintic {
            c0: l.c0,
            c1: l.c1,
            ..Self::zero()
        }
    }
}

impl<T: Additive> From<Quadratic<T>> for Quintic<T> {
    fn from(q: Quadratic<T>) -> Self {
        Quintic {
            c0: q.c0,
            c1: q.c1,
            c2: q.c2,
            ..Self::zero()
        }
    }
}

impl<T: Additive> From<Cubic<T>> for Quintic<T> {
    fn from(c: Cubic<T>) -> Self {
        Quintic {
            c0: c.c0,
            c1: c.c1,
            c2: c.c2,
            c3: c.c3,
            ..Self::zero()
        }
    }
}

impl<T> Functor for Quintic<T> {
    type Param = T;
    type Output<B> = Quintic<B>;
    fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> Quintic<B> {
        Quintic {
            c0: f(self.c0),
            c1: f(self.c1),
            c2: f(self.c2),
            c3: f(self.c3),
            c4: f(self.c4),
            c5: f(self.c5),
        }
    }
}

#[inline]
pub fn unit_hermite5<T>(
    p_start: T,
    v_start: T,
    a_start: T,
    p_end: T,
    v_end: T,
    a_end: T,
) -> Quintic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Field,
{
    Quintic {
        c0: p_start.clone(),
        c1: v_start.clone(),
        c2: a_start.qscaled(1, 2),
        c3: iter_sum_reduce([
            p_start.iscaled(-10),
            p_end.iscaled(10),
            v_start.iscaled(-6),
            v_end.iscaled(-4),
            a_start.qscaled(-3, 2),
            a_end.qscaled(1, 2),
        ]),
        c4: iter_sum_reduce([
            p_start.iscaled(15),
            p_end.iscaled(-15),
            v_start.iscaled(8),
            v_end.iscaled(7),
            a_start.qscaled(3, 2),
            a_end.iscaled(-1),
        ]),
        c5: iter_sum_reduce([
            p_start.iscale(-6),
            p_end.iscale(6),
            v_start.iscale(-3),
            v_end.iscale(-3),
            a_start.qscale(-1, 2),
            a_end.qscale(1, 2),
        ]),
    }
}
