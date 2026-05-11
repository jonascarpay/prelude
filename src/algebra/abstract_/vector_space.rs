use super::additive::Additive;
use super::ring::Ring;

pub trait VectorSpace: Additive
where
    Self::Over: Ring,
{
    type Over;

    /// Scalar multiplication such that
    ///   - v.scale(a).scale(b) = v.scale(a.mult(b))
    ///   - v.scale(one()) = v
    ///   - v.plus(u).scale(a) = v.scale(a).plus(u.scale(a))
    ///   - v.scale(a.plus(b)) = v.scale(a).plus(v.scale(b))
    fn scale(self, c: Self::Over) -> Self;
    // TODO specify left or right module? Or what side we scale on?
}

impl<R, A, B> VectorSpace for (A, B)
where
    R: Ring + Clone,
    A: VectorSpace<Over = R>,
    B: VectorSpace<Over = R>,
{
    type Over = R;

    fn scale(self, c: Self::Over) -> Self {
        let (a, b) = self;
        (a.scale(c.clone()), b.scale(c))
    }
}

impl<R, A, B, C> VectorSpace for (A, B, C)
where
    R: Ring + Clone,
    A: VectorSpace<Over = R>,
    B: VectorSpace<Over = R>,
    C: VectorSpace<Over = R>,
{
    type Over = R;

    fn scale(self, c: Self::Over) -> Self {
        let (a, b, cc) = self;
        (a.scale(c.clone()), b.scale(c.clone()), cc.scale(c))
    }
}

impl<R, A, B, C, D> VectorSpace for (A, B, C, D)
where
    R: Ring + Clone,
    A: VectorSpace<Over = R>,
    B: VectorSpace<Over = R>,
    C: VectorSpace<Over = R>,
    D: VectorSpace<Over = R>,
{
    type Over = R;

    fn scale(self, s: Self::Over) -> Self {
        let (a, b, c, d) = self;
        (
            a.scale(s.clone()),
            b.scale(s.clone()),
            c.scale(s.clone()),
            d.scale(s),
        )
    }
}

/// Emit `Mul<Self::Over>` impl that forwards to `VectorSpace::scale`.
///
/// Usage: `impl_vector_space_ops!([T: Ring + Copy] V2<T>);`
#[macro_export]
macro_rules! impl_vector_space_ops {
    ([$($g:tt)*] $t:ty) => {
        impl<$($g)*> ::core::ops::Mul<<Self as $crate::algebra::abstract_::VectorSpace>::Over> for $t {
            type Output = Self;
            fn mul(self, c: <Self as $crate::algebra::abstract_::VectorSpace>::Over) -> Self {
                <Self as $crate::algebra::abstract_::VectorSpace>::scale(self, c)
            }
        }
    };
}

impl<R, const N: usize> VectorSpace for [R; N]
where
    R: Ring + Clone,
{
    type Over = R;

    fn scale(self, c: Self::Over) -> Self {
        self.map(|x| x.mult(c.clone()))
    }
}

pub fn linear_combination<T: VectorSpace, const N: usize>(
    weights: [T::Over; N],
    vecs: [T; N],
) -> T {
    use std::iter::zip;
    zip(weights, vecs).fold(T::zero(), |s, (c, a)| s.plus(a.scale(c)))
}
