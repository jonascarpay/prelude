use crate::algebra::polynomial::spline::Spline;

use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Curve, DifferentiableCurve, Ring,
    VectorSpace,
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

impl<T: Ring> Cubic<T> {
    pub fn x3() -> Self {
        Cubic {
            c3: T::one(),
            ..Self::zero()
        }
    }
    pub fn x2() -> Self {
        Cubic {
            c3: T::one(),
            ..Self::zero()
        }
    }
    pub fn x1() -> Self {
        Cubic {
            c1: T::one(),
            ..Self::zero()
        }
    }

    /// Construct a cubic from the factored from `a(x - r1)(x - r2)(x - r3)`
    pub fn from_roots(a: T, r1: T, r2: T, r3: T) -> Self
    where
        T: Copy, // TODO Clone
    {
        // a(x - r1)(x - r2)(x - r3)
        // a(x - r1)(x^2 - r2 x - r3 x + r2 r3)
        // ax^3 - a r2 x^2 - a r3 x^2 + a r2 r3 x - a r1 x^2 + a r1 r2 x + a r1 r3 x - a r1 r2 r3
        //
        // a x^3
        // a (- r1 - r2 - r3) x^2
        // a (r2 r3 + a r1 r2 + a r1 r3) x
        // a (- r1 r2 r3)
        Cubic {
            c0: r1.mult(r2).mult(r3).negate(),
            c1: (r1.mult(r2).plus(r2.mult(r3)).plus(r1.mult(r3))),
            c2: (r1.negate().minus(r2).minus(r3).mult(a)),
            c3: a,
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

impl<T: VectorSpace> Curve for Cubic<T> {
    type Domain = T::Scalar;
    type Codomain = T;
    fn evaluate(self, x: T::Scalar) -> T {
        self.c0
            .plus(self.c1.scale(x.clone()))
            .plus(self.c2.scale(x.clone().squared()))
            .plus(self.c3.scale(x.cubed()))
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

/// A cubic Bezier polynomial, given a start point, two control points, and an end point.
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

/// A C1-continuous Bezier spline, i.e. continuous velocity.
/// Parameters are
///   - the start point
///   - the start point's outward tangent control point
///   - a list of (inward tangent control point, knot) pairs.
pub fn bezier_spline_c1<const N: usize, T: VectorSpace>(
    p_start: T,
    p_start_out: T,
    controls: [(T, T); N],
) -> Spline<N, Cubic<T>> {
    let mut p_prev = p_start.clone();
    let mut p_prev_out = p_start_out.clone();

    Spline {
        curves: controls.map(|(p_in, p)| {
            let c = bezier3(p_prev.clone(), p_start_out.clone(), p_in.clone(), p.clone());
            p_prev = p.clone();
            let d = p.clone().minus(p_in);
            p_prev_out = p.plus(d);
            c
        }),
    }
}

// TODO bezier_spline_g1 (tangent-continuous), takes a list [(T, T, T::Scalar)]

/// A C0-continuous Bezier spline, i.e. continuous position.
/// Parameters are
///   - the start point
///   - the start point's outward tangent control point
///   - a list of (inward tangent control point, knot, outward tangent control point) pairs.
///
/// For the last knot, the outward tangent control point is ignored.
pub fn bezier_spline_c0<const N: usize, T: VectorSpace>(
    p_start: T,
    p_start_out: T,
    controls: [(T, T, T); N],
) -> Spline<N, Cubic<T>> {
    let mut p_prev = p_start.clone();
    let mut p_prev_out = p_start_out.clone();

    Spline {
        curves: controls.map(|(p_in, p, p_out)| {
            let c = bezier3(p_prev.clone(), p_start_out.clone(), p_in.clone(), p.clone());
            p_prev = p;
            p_prev_out = p_out;
            c
        }),
    }
}
