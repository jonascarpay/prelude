use super::additive::Additive;

pub trait Ring: Additive + Sized + Clone {
    /// An associative operation, distributive w.r.t. `plus`
    fn mult(self, rhs: Self) -> Self;

    /// Integer homomorphism
    fn from_integer(i: isize) -> Self;

    /// Identity element for `mult` such that `zero() != one()`
    fn one() -> Self {
        Self::from_integer(1)
    }

    fn squared(self) -> Self {
        self.clone().mult(self)
    }

    fn cubed(self) -> Self {
        self.clone().mult(self.clone()).mult(self.clone())
    }

    fn imult(self, rhs: isize) -> Self {
        self.mult(Self::from_integer(rhs))
    }

    fn succ(self) -> Self {
        self.plus(Self::one())
    }

    // TODO implement in macros
    fn incr(&mut self) {
        *self = self.clone().succ();
    }
}

pub fn one<R: Ring>() -> R {
    R::one()
}

macro_rules! impl_ring_modular {
    ($t:ty) => {
        impl Ring for $t {
            fn mult(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }
            fn one() -> Self {
                1
            }
            fn from_integer(i: isize) -> Self {
                i as $t
            }
        }
    };
}

macro_rules! impl_ring_float {
    ($t:ty) => {
        impl Ring for $t {
            fn mult(self, rhs: Self) -> Self {
                self * rhs
            }
            fn one() -> Self {
                1.0
            }
            fn from_integer(i: isize) -> Self {
                i as $t
            }
        }
    };
}

/// Emit `Mul<Self>` impl that forwards to `Ring::mult`.
///
/// Usage: `impl_ring_ops!([T: Ring + Copy] Polynomial<T>);`
#[macro_export]
macro_rules! impl_ring_ops {
    ([$($g:tt)*] $t:ty) => {
        impl<$($g)*> ::core::ops::Mul for $t {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self {
                <Self as $crate::algebra::abstract_::Ring>::mult(self, rhs)
            }
        }
    };
}

impl_ring_modular!(i8);
impl_ring_modular!(i16);
impl_ring_modular!(i32);
impl_ring_modular!(i64);
impl_ring_modular!(i128);
impl_ring_modular!(isize);
impl_ring_modular!(u8);
impl_ring_modular!(u16);
impl_ring_modular!(u32);
impl_ring_modular!(u64);
impl_ring_modular!(u128);
impl_ring_modular!(usize);
impl_ring_float!(f32);
impl_ring_float!(f64);
