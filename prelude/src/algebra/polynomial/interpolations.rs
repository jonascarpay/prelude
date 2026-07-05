use crate::algebra::{
    abstract_::{additive::iter_sum_reduce, field::Field, VectorSpace},
    polynomial::dense::{Cubic, Quadratic, Quintic},
    Ring,
};

pub fn bezier2<T>(p0: T, p1: T, p2: T) -> Quadratic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Quadratic::from_coefficients([
        p0.clone(),
        p0.iscaled(-2).plus(p1.iscaled(1)),
        p0.plus(p1.iscale(-2)).plus(p2),
    ])
}

/// A cubic Bezier polynomial on the [0, 1] domain, given a start point, two control points, and an end point.
pub fn bezier3<T>(p0: T, p1: T, p2: T, p3: T) -> Cubic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Cubic::from_coefficients([
        p0.clone(),
        p0.iscaled(-3).plus(p1.iscaled(3)),
        p0.iscaled(3).plus(p1.iscaled(-6)).plus(p2.iscaled(3)),
        p0.negate().plus(p1.iscale(3)).plus(p2.iscale(-3).plus(p3)),
    ])
}

#[inline(always)]
/// A cubic hermite polynomial on the [0, 1] interval, with given boundary conditions
pub fn unit_hermite3<T>(p_start: T, v_start: T, p_end: T, v_end: T) -> Cubic<T>
where
    T: VectorSpace + Clone,
    T::Scalar: Ring,
{
    Cubic::from_coefficients([
        p_start.clone(),
        v_start.clone(),
        (p_start.iscaled(-3))
            .plus(p_end.iscaled(3))
            .plus(v_start.iscaled(-2))
            .plus(v_end.negated()),
        (p_start.iscale(2))
            .plus(p_end.iscale(-2))
            .plus(v_start)
            .plus(v_end),
    ])
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
    Quintic::from_coefficients([
        p_start.clone(),
        v_start.clone(),
        a_start.qscaled(1, 2),
        iter_sum_reduce([
            p_start.iscaled(-10),
            p_end.iscaled(10),
            v_start.iscaled(-6),
            v_end.iscaled(-4),
            a_start.qscaled(-3, 2),
            a_end.qscaled(1, 2),
        ]),
        iter_sum_reduce([
            p_start.iscaled(15),
            p_end.iscaled(-15),
            v_start.iscaled(8),
            v_end.iscaled(7),
            a_start.qscaled(3, 2),
            a_end.iscaled(-1),
        ]),
        iter_sum_reduce([
            p_start.iscale(-6),
            p_end.iscale(6),
            v_start.iscale(-3),
            v_end.iscale(-3),
            a_start.qscale(-1, 2),
            a_end.qscale(1, 2),
        ]),
    ])
}
