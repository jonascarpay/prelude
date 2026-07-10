use super::additive::Additive;

pub trait Ring: Additive + Sized + Clone {
    /// Identity element for `mult` such that `zero() != one()`
    const ONE: Self;

    /// An associative operation, distributive w.r.t. `plus`
    fn mult(self, rhs: Self) -> Self;

    /// Integer homomorphism
    fn from_integer(i: isize) -> Self;

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
        self.plus(Self::ONE)
    }

    // TODO implement in macros
    fn incr(&mut self) {
        *self = self.clone().succ();
    }
}

pub const fn one<R: Ring>() -> R {
    R::ONE
}

impl<A, B> Ring for (A, B)
where
    A: Ring,
    B: Ring,
{
    const ONE: Self = (A::ONE, B::ONE);

    fn mult(self, rhs: Self) -> Self {
        let (a0, a1) = self;
        let (b0, b1) = rhs;
        (a0.mult(b0), a1.mult(b1))
    }

    fn from_integer(i: isize) -> Self {
        (A::from_integer(i), B::from_integer(i))
    }
}

impl<A, B, C> Ring for (A, B, C)
where
    A: Ring,
    B: Ring,
    C: Ring,
{
    const ONE: Self = (A::ONE, B::ONE, C::ONE);

    fn from_integer(i: isize) -> Self {
        (A::from_integer(i), B::from_integer(i), C::from_integer(i))
    }

    fn mult(self, rhs: Self) -> Self {
        let (a0, a1, a2) = self;
        let (b0, b1, b2) = rhs;
        (a0.mult(b0), a1.mult(b1), a2.mult(b2))
    }
}

impl<A, B, C, D> Ring for (A, B, C, D)
where
    A: Ring,
    B: Ring,
    C: Ring,
    D: Ring,
{
    const ONE: Self = (A::ONE, B::ONE, C::ONE, D::ONE);

    fn mult(self, rhs: Self) -> Self {
        let (a0, a1, a2, a3) = self;
        let (b0, b1, b2, b3) = rhs;
        (a0.mult(b0), a1.mult(b1), a2.mult(b2), a3.mult(b3))
    }

    fn from_integer(i: isize) -> Self {
        (
            A::from_integer(i),
            B::from_integer(i),
            C::from_integer(i),
            D::from_integer(i),
        )
    }
}

impl<T: Ring, const N: usize> Ring for [T; N] {
    const ONE: Self = std::array::from_fn(|_| T::ONE);

    fn mult(self, rhs: Self) -> Self {
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        std::array::from_fn(|_| lhs.next().unwrap().mult(rhs.next().unwrap()))
    }

    fn from_integer(i: isize) -> Self {
        std::array::from_fn(|_| T::from_integer(i))
    }
}

macro_rules! impl_ring_modular {
    ($t:ty) => {
        impl Ring for $t {
            const ONE: Self = 1;
            fn mult(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
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
            const ONE: Self = 1.0;
            fn mult(self, rhs: Self) -> Self {
                self * rhs
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
