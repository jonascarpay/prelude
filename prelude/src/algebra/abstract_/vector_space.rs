use super::additive::Additive;
use super::ring::Ring;

/// This is technically a _module_, since it's constrained to Ring instead of Field.
pub trait VectorSpace: Additive
where
    Self::Scalar: Ring,
{
    type Scalar;

    /// Scalar multiplication such that
    ///   - v.scale(a).scale(b) = v.scale(a.mult(b))
    ///   - v.scale(one()) = v
    ///   - v.plus(u).scale(a) = v.scale(a).plus(u.scale(a))
    ///   - v.scale(a.plus(b)) = v.scale(a).plus(v.scale(b))
    fn scale(self, c: Self::Scalar) -> Self;
    // TODO specify left or right module? Or what side we scale on?
}

impl<R, A, B> VectorSpace for (A, B)
where
    R: Ring + Clone,
    A: VectorSpace<Scalar = R>,
    B: VectorSpace<Scalar = R>,
{
    type Scalar = R;

    fn scale(self, c: Self::Scalar) -> Self {
        let (a, b) = self;
        (a.scale(c.clone()), b.scale(c))
    }
}

impl<R, A, B, C> VectorSpace for (A, B, C)
where
    R: Ring + Clone,
    A: VectorSpace<Scalar = R>,
    B: VectorSpace<Scalar = R>,
    C: VectorSpace<Scalar = R>,
{
    type Scalar = R;

    fn scale(self, c: Self::Scalar) -> Self {
        let (a, b, cc) = self;
        (a.scale(c.clone()), b.scale(c.clone()), cc.scale(c))
    }
}

impl<R, A, B, C, D> VectorSpace for (A, B, C, D)
where
    R: Ring + Clone,
    A: VectorSpace<Scalar = R>,
    B: VectorSpace<Scalar = R>,
    C: VectorSpace<Scalar = R>,
    D: VectorSpace<Scalar = R>,
{
    type Scalar = R;

    fn scale(self, s: Self::Scalar) -> Self {
        let (a, b, c, d) = self;
        (
            a.scale(s.clone()),
            b.scale(s.clone()),
            c.scale(s.clone()),
            d.scale(s),
        )
    }
}

/// Emit `Mul<Self::Scalar>` and `Div<Self::Scalar>` impls that forward to
/// `VectorSpace::scale`. The `Div` impl is only callable when `Self::Scalar: Field`.
///
/// Usage: `impl_vector_space_ops!([T: Ring + Copy] V2<T>);`
///
/// TODO: split into separate `impl_vector_space_mul` and `impl_vector_space_div`
/// macros. The combined form fails for concrete non-Field scalars (e.g. `Unorm8`),
/// because rustc evaluates the unconditional `where Scalar: Field` bound on the
/// `Div` impl and rejects it. With generic scalars the bound is conditional and
/// compiles fine.
#[macro_export]
macro_rules! impl_vector_space_ops {
    ([$($g:tt)*] $t:ty) => {
        impl<$($g)*> ::core::ops::Mul<<Self as $crate::algebra::abstract_::VectorSpace>::Scalar> for $t {
            type Output = Self;
            fn mul(self, c: <Self as $crate::algebra::abstract_::VectorSpace>::Scalar) -> Self {
                <Self as $crate::algebra::abstract_::VectorSpace>::scale(self, c)
            }
        }

        impl<$($g)*> ::core::ops::Div<<Self as $crate::algebra::abstract_::VectorSpace>::Scalar> for $t
        where
            <Self as $crate::algebra::abstract_::VectorSpace>::Scalar:
                $crate::algebra::abstract_::field::Field,
        {
            type Output = Self;
            fn div(self, c: <Self as $crate::algebra::abstract_::VectorSpace>::Scalar) -> Self {
                <Self as $crate::algebra::abstract_::VectorSpace>::scale(
                    self,
                    $crate::algebra::abstract_::field::Field::recip(c),
                )
            }
        }
    };
}

impl<R, const N: usize> VectorSpace for [R; N]
where
    R: Ring + Clone,
{
    type Scalar = R;

    fn scale(self, c: Self::Scalar) -> Self {
        self.map(|x| x.mult(c.clone()))
    }
}

pub fn linear_combination<T: VectorSpace, const N: usize>(
    weights: [T::Scalar; N],
    vecs: [T; N],
) -> T {
    use std::iter::zip;
    zip(weights, vecs).fold(T::zero(), |s, (c, a)| s.plus(a.scale(c)))
}
