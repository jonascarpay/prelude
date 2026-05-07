use super::additive::Additive;

pub trait Ring: Additive + Sized {
    /// An associative operation, distributive w.r.t. `plus`
    fn mult(self, rhs: Self) -> Self;

    /// Identity element for `mult` such that `zero() != one()`
    fn one() -> Self {
        Self::from_integer(1)
    }

    fn from_integer(i: isize) -> Self;
}

impl<A, B> Ring for (A, B)
where
    A: Ring,
    B: Ring,
{
    fn mult(self, rhs: Self) -> Self {
        let (a0, a1) = self;
        let (b0, b1) = rhs;
        (a0.mult(b0), a1.mult(b1))
    }

    fn one() -> Self {
        (A::one(), B::one())
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
    fn mult(self, rhs: Self) -> Self {
        let (a0, a1, a2) = self;
        let (b0, b1, b2) = rhs;
        (a0.mult(b0), a1.mult(b1), a2.mult(b2))
    }

    fn one() -> Self {
        (A::one(), B::one(), C::one())
    }
    fn from_integer(i: isize) -> Self {
        (A::from_integer(i), B::from_integer(i), C::from_integer(i))
    }
}

impl<A, B, C, D> Ring for (A, B, C, D)
where
    A: Ring,
    B: Ring,
    C: Ring,
    D: Ring,
{
    fn mult(self, rhs: Self) -> Self {
        let (a0, a1, a2, a3) = self;
        let (b0, b1, b2, b3) = rhs;
        (a0.mult(b0), a1.mult(b1), a2.mult(b2), a3.mult(b3))
    }

    fn one() -> Self {
        (A::one(), B::one(), C::one(), D::one())
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
    fn mult(self, rhs: Self) -> Self {
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        std::array::from_fn(|_| lhs.next().unwrap().mult(rhs.next().unwrap()))
    }

    fn one() -> Self {
        std::array::from_fn(|_| T::one())
    }

    fn from_integer(i: isize) -> Self {
        std::array::from_fn(|_| T::from_integer(i))
    }
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
