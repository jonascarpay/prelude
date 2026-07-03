use crate::algebra::abstract_::field::Field;

use super::additive::Additive;
use super::ring::Ring;

/// This is technically a _module_, since it's constrained to Ring instead of Field.
pub trait VectorSpace: Additive + Clone
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

    fn scaled(&self, c: Self::Scalar) -> Self {
        self.clone().scale(c)
    }
    fn iscale(self, c: isize) -> Self {
        self.scale(Self::Scalar::from_integer(c))
    }
    fn iscaled(&self, c: isize) -> Self {
        self.clone().iscale(c)
    }
    fn qscale(self, p: isize, q: isize) -> Self
    where
        Self::Scalar: Field,
    {
        self.scale(Self::Scalar::from_rational(p, q))
    }
    fn qscaled(&self, p: isize, q: isize) -> Self
    where
        Self::Scalar: Field,
    {
        self.clone().qscale(p, q)
    }
    // TODO specify left or right module? Or what side we scale on?
}

// TODO: There is possible further work here, of affine spaces and convex sets/combinations.
// This is useful for e.g. colors, that shouldn't leave their gamut.
// https://en.wikipedia.org/wiki/Convex_combination

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

impl<R, const N: usize> VectorSpace for [R; N]
where
    R: Ring + Clone,
{
    type Scalar = R;

    fn scale(self, c: Self::Scalar) -> Self {
        self.map(|x| x.mult(c.clone()))
    }
}

#[macro_export]
macro_rules! impl_trivial_vector_space {
    ($t:ty) => {
        impl VectorSpace for $t {
            type Scalar = $t;
            fn scale(self, s: Self) -> Self {
                self.mult(s)
            }
        }
    };
}

/// Emit `Mul<Self::Scalar>` impl that forwards to `VectorSpace::scale`.
///
/// Usage: `impl_vector_space_mul!([T: Ring + Copy] V2<T>);`
#[macro_export]
macro_rules! impl_vector_space_mul {
    ([$($g:tt)*] $t:ty) => {
        impl<$($g)*> ::core::ops::Mul<<Self as $crate::algebra::abstract_::VectorSpace>::Scalar> for $t {
            type Output = Self;
            fn mul(self, c: <Self as $crate::algebra::abstract_::VectorSpace>::Scalar) -> Self {
                <Self as $crate::algebra::abstract_::VectorSpace>::scale(self, c)
            }
        }
    };
}

/// Emit `Div<Self::Scalar>` impl that forwards to `VectorSpace::scale` via `Field::recip`.
/// Requires `Self::Scalar: Field` — only call this when the scalar is generic enough
/// to be a Field, or when you know it concretely implements Field.
///
/// Usage: `impl_vector_space_div!([T: Field + Copy] V2<T>);`
#[macro_export]
macro_rules! impl_vector_space_div {
    ([$($g:tt)*] $t:ty) => {
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

/// Emit both `Mul<Self::Scalar>` and `Div<Self::Scalar>` impls. The `Div` impl is
/// only callable when `Self::Scalar: Field`, so this form requires a scalar that
/// is generic over Field or known concretely to implement Field — use
/// `impl_vector_space_mul!` alone if your scalar is not a Field (e.g. `Unorm8`).
///
/// Usage: `impl_vector_space_ops!([T: Ring + Copy] V2<T>);`
/// TODO: probably just drop this and have implementors invoke both macros manually
#[macro_export]
macro_rules! impl_vector_space_ops {
    ([$($g:tt)*] $t:ty) => {
        $crate::impl_vector_space_mul!([$($g)*] $t);
        $crate::impl_vector_space_div!([$($g)*] $t);
    };
}

impl_trivial_vector_space!(i8);
impl_trivial_vector_space!(i16);
impl_trivial_vector_space!(i32);
impl_trivial_vector_space!(i64);
impl_trivial_vector_space!(i128);
impl_trivial_vector_space!(isize);
impl_trivial_vector_space!(u8);
impl_trivial_vector_space!(u16);
impl_trivial_vector_space!(u32);
impl_trivial_vector_space!(u64);
impl_trivial_vector_space!(u128);
impl_trivial_vector_space!(usize);
impl_trivial_vector_space!(f32);
impl_trivial_vector_space!(f64);
