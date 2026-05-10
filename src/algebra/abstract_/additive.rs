/// An additive group
pub trait Additive: Sized {
    /// An associative, commutative operation
    fn plus(self, rhs: Self) -> Self;

    /// The identity element for `plus`
    fn zero() -> Self;

    /// The inverse element for `plus`
    fn negate(self) -> Self;

    fn minus(self, rhs: Self) -> Self {
        self.plus(rhs.negate())
    }

    fn is_zero(&self) -> bool;
}

#[inline]
pub fn iter_sum<T: Additive, I: Iterator<Item = T>>(mut iter: I) -> T {
    let mut res = T::zero();
    while let Some(c) = iter.next() {
        res = res.plus(c);
    }
    res
}

impl Additive for () {
    fn plus(self, _rhs: Self) -> Self {}
    fn zero() -> Self {}
    fn negate(self) -> Self {}
    fn is_zero(&self) -> bool {
        true
    }
}

impl<A, B> Additive for (A, B)
where
    A: Additive,
    B: Additive,
{
    fn plus(self, rhs: Self) -> Self {
        let (a0, a1) = self;
        let (b0, b1) = rhs;
        (a0.plus(b0), a1.plus(b1))
    }
    fn minus(self, rhs: Self) -> Self {
        let (a0, a1) = self;
        let (b0, b1) = rhs;
        (a0.minus(b0), a1.minus(b1))
    }

    fn zero() -> Self {
        (A::zero(), B::zero())
    }

    fn negate(self) -> Self {
        let (a0, a1) = self;
        (a0.negate(), a1.negate())
    }

    fn is_zero(&self) -> bool {
        let (a0, a1) = self;
        a0.is_zero() && a1.is_zero()
    }
}

impl<A, B, C> Additive for (A, B, C)
where
    A: Additive,
    B: Additive,
    C: Additive,
{
    fn plus(self, rhs: Self) -> Self {
        let (a0, a1, a2) = self;
        let (b0, b1, b2) = rhs;
        (a0.plus(b0), a1.plus(b1), a2.plus(b2))
    }
    fn minus(self, rhs: Self) -> Self {
        let (a0, a1, a2) = self;
        let (b0, b1, b2) = rhs;
        (a0.minus(b0), a1.minus(b1), a2.minus(b2))
    }

    fn zero() -> Self {
        (A::zero(), B::zero(), C::zero())
    }

    fn negate(self) -> Self {
        let (a0, a1, a2) = self;
        (a0.negate(), a1.negate(), a2.negate())
    }
    fn is_zero(&self) -> bool {
        let (a0, a1, a2) = self;
        a0.is_zero() && a1.is_zero() && a2.is_zero()
    }
}

impl<A, B, C, D> Additive for (A, B, C, D)
where
    A: Additive,
    B: Additive,
    C: Additive,
    D: Additive,
{
    fn plus(self, rhs: Self) -> Self {
        let (a0, a1, a2, a3) = self;
        let (b0, b1, b2, b3) = rhs;
        (a0.plus(b0), a1.plus(b1), a2.plus(b2), a3.plus(b3))
    }
    fn minus(self, rhs: Self) -> Self {
        let (a0, a1, a2, a3) = self;
        let (b0, b1, b2, b3) = rhs;
        (a0.minus(b0), a1.minus(b1), a2.minus(b2), a3.minus(b3))
    }

    fn zero() -> Self {
        (A::zero(), B::zero(), C::zero(), D::zero())
    }

    fn negate(self) -> Self {
        let (a0, a1, a2, a3) = self;
        (a0.negate(), a1.negate(), a2.negate(), a3.negate())
    }

    fn is_zero(&self) -> bool {
        let (a0, a1, a2, a3) = self;
        a0.is_zero() && a1.is_zero() && a2.is_zero() && a3.is_zero()
    }
}

impl<T: Additive, const N: usize> Additive for [T; N] {
    fn plus(self, rhs: Self) -> Self {
        // Confirmed this unrolls and vectorizes cleanly
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        std::array::from_fn(|_| lhs.next().unwrap().plus(rhs.next().unwrap()))
    }
    fn minus(self, rhs: Self) -> Self {
        // Confirmed this unrolls and vectorizes cleanly
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        std::array::from_fn(|_| lhs.next().unwrap().minus(rhs.next().unwrap()))
    }

    fn zero() -> Self {
        std::array::from_fn(|_| T::zero())
    }

    fn negate(self) -> Self {
        self.map(T::negate)
    }

    fn is_zero(&self) -> bool {
        self.iter().all(T::is_zero)
    }
}

macro_rules! impl_additive_modular {
    ($t:ty) => {
        impl Additive for $t {
            fn plus(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
            fn minus(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }
            fn zero() -> Self {
                0
            }
            fn negate(self) -> Self {
                self.wrapping_neg()
            }
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
    };
}

macro_rules! impl_additive_float {
    ($t:ty) => {
        impl Additive for $t {
            fn plus(self, rhs: Self) -> Self {
                self + rhs
            }
            fn minus(self, rhs: Self) -> Self {
                self - rhs
            }
            fn zero() -> Self {
                0.0
            }
            fn negate(self) -> Self {
                -self
            }
            fn is_zero(&self) -> bool {
                *self == 0.0
            }
        }
    };
}

/// Emit `Add`, `Sub`, `Neg` impls that forward to `Additive`.
///
/// Usage: `impl_additive_ops!([T: Additive] V2<T>);`
#[macro_export]
macro_rules! impl_additive_ops {
    ([$($g:tt)*] $t:ty) => {
        impl<$($g)*> ::core::ops::Add for $t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                <Self as $crate::algebra::abstract_::Additive>::plus(self, rhs)
            }
        }
        impl<$($g)*> ::core::ops::Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                <Self as $crate::algebra::abstract_::Additive>::minus(self, rhs)
            }
        }
        impl<$($g)*> ::core::ops::Neg for $t {
            type Output = Self;
            fn neg(self) -> Self {
                <Self as $crate::algebra::abstract_::Additive>::negate(self)
            }
        }
    };
}

impl_additive_modular!(i8);
impl_additive_modular!(i16);
impl_additive_modular!(i32);
impl_additive_modular!(i64);
impl_additive_modular!(i128);
impl_additive_modular!(isize);
impl_additive_modular!(u8);
impl_additive_modular!(u16);
impl_additive_modular!(u32);
impl_additive_modular!(u64);
impl_additive_modular!(u128);
impl_additive_modular!(usize);
impl_additive_float!(f32);
impl_additive_float!(f64);
