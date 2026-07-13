use crate::algebra::{abstract_::field::Field, Additive, Ring};

/// A right-module: `scale(self, c)` is `v · c`.
///
/// Right-handed so that the coherence-legal operator is `vector * scalar`
/// (the vector, a local type, sits in `Self` position). For commutative
/// scalars this coincides with the left-module.
pub trait Module<R: Ring>: Additive + Clone {
    fn scale(self, c: R) -> Self;
}

impl<R: Ring> Module<R> for R {
    fn scale(self, c: R) -> Self {
        self.mult(c)
    }
}

impl<R: Ring, T: Module<R>, const N: usize> Module<R> for [T; N] {
    fn scale(self, c: R) -> Self {
        self.map(|i| i.scale(c.clone()))
    }
}

impl<R: Ring, A: Module<R>, B: Module<R>> Module<R> for (A, B) {
    fn scale(self, c: R) -> Self {
        (self.0.scale(c.clone()), self.1.scale(c))
    }
}

// Formally, should be over a Field, but we want to put iscale here
pub trait VectorSpace: Module<Self::Scalar> {
    type Scalar: Ring;

    fn iscale(self, c: isize) -> Self {
        self.scale(Self::Scalar::from_integer(c))
    }
    fn qscale(self, p: isize, q: isize) -> Self
    where
        Self::Scalar: Field,
    {
        self.scale(Self::Scalar::from_rational(p, q))
    }
}

/// Emit `vector * scalar` for every ring the vector is a right-`Module` over,
/// forwarding to `Module::scale`. Keyed on the vector (the local type), so it is
/// coherence-legal and blankets over all scalars `R`.
///
/// Use this OR a `Ring`-derived `Mul` on the same type, not both — they overlap
/// at `R = Self` for ring-like types.
///
/// Usage: `impl_module_mul!([T: Ring] V2<T>);`
#[macro_export]
macro_rules! impl_module_mul {
    ([$($g:tt)*] $vec:ty) => {
        impl<$($g)* R: $crate::algebra::Ring> ::core::ops::Mul<R> for $vec
        where
            $vec: $crate::algebra::abstract_::module::Module<R>,
        {
            type Output = $vec;
            #[inline]
            fn mul(self, c: R) -> $vec {
                $crate::algebra::abstract_::module::Module::scale(self, c)
            }
        }
    };
}

/// Emit `vector / scalar` for every `Field` the vector is a right-`Module` over,
/// forwarding to `Module::scale` via `Field::recip`.
///
/// Usage: `impl_module_div!([T: Ring] V2<T>);`
#[macro_export]
macro_rules! impl_module_div {
    ([$($g:tt)*] $vec:ty) => {
        impl<$($g)* R: $crate::algebra::abstract_::field::Field> ::core::ops::Div<R> for $vec
        where
            $vec: $crate::algebra::abstract_::module::Module<R>,
        {
            type Output = $vec;
            #[inline]
            fn div(self, c: R) -> $vec {
                $crate::algebra::abstract_::module::Module::scale(
                    self,
                    $crate::algebra::abstract_::field::Field::recip(c),
                )
            }
        }
    };
}
