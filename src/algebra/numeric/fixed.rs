use std::ops::{Shl, Shr};

use crate::{
    algebra::abstract_::{
        euclidean_ring::EuclideanRing, field::Field, Additive, Ring, VectorSpace,
    },
    impl_additive_ops, impl_ring_ops, impl_vector_space_ops,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed<T, const FRAC_BITS: u32> {
    raw: T,
}

impl<T, const FRAC_BITS: u32> Fixed<T, FRAC_BITS> {
    pub fn from_raw(raw: T) -> Self {
        Fixed { raw }
    }

    pub fn into_raw(self) -> T {
        self.raw
    }
}

impl<T: FixedBase, const FRAC_BITS: u32> Fixed<T, FRAC_BITS> {
    pub const MIN: Self = Fixed { raw: T::MIN };
    pub const MAX: Self = Fixed { raw: T::MAX };
    pub const ZERO: Self = Fixed { raw: T::ZERO };
    pub const EPSILON: Self = Fixed { raw: T::ONE };

    pub fn from_f64(x: f64) -> Self {
        let scale = (2.0_f64).powi(FRAC_BITS as i32);
        Fixed {
            raw: T::from_f64(x * scale),
        }
    }

    pub fn to_f64(self) -> f64 {
        let scale = (2.0_f64).powi(FRAC_BITS as i32);
        self.raw.to_f64() / scale
    }

    /// Integer part. Floors toward negative infinity, such that (a.trunc() << FRAC_BITS + a.fract) = a.into_raw().
    pub fn trunc(self) -> T {
        self.raw >> FRAC_BITS
    }

    /// Fractional part, in `[0, 2^FRAC_BITS)`.
    pub fn fract(self) -> T {
        self.raw.clone().minus(self.trunc() << FRAC_BITS)
    }
}

impl<T: Additive, const FRAC_BITS: u32> Additive for Fixed<T, FRAC_BITS> {
    fn plus(self, rhs: Self) -> Self {
        Fixed {
            raw: self.raw.plus(rhs.raw),
        }
    }

    fn zero() -> Self {
        Fixed { raw: T::zero() }
    }

    fn negate(self) -> Self {
        Fixed {
            raw: self.raw.negate(),
        }
    }

    fn is_zero(&self) -> bool {
        self.raw.is_zero()
    }

    fn minus(self, rhs: Self) -> Self {
        Fixed {
            raw: self.raw.minus(rhs.raw),
        }
    }
}

impl<T: Ring, const FRAC_BITS: u32> VectorSpace for Fixed<T, FRAC_BITS> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Fixed {
            raw: self.raw.mult(c),
        }
    }
}

pub trait FixedBase: Ring + Shl<u32, Output = Self> + Shr<u32, Output = Self> {
    type Wide: EuclideanRing + Shl<u32, Output = Self::Wide> + Shr<u32, Output = Self::Wide>;
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;
    fn widen(self) -> Self::Wide;
    fn narrow(wide: Self::Wide) -> Self;
    fn from_f64(x: f64) -> Self;
    fn to_f64(self) -> f64;
}

impl<T: FixedBase, const FRAC_BITS: u32> Ring for Fixed<T, FRAC_BITS> {
    fn mult(self, rhs: Self) -> Self {
        let a = self.raw.widen();
        let b = rhs.raw.widen();
        Fixed {
            raw: T::narrow(a.mult(b) >> FRAC_BITS),
        }
    }

    fn from_integer(i: isize) -> Self {
        Fixed {
            raw: T::from_integer(i) << FRAC_BITS,
        }
    }

    fn one() -> Self {
        Fixed {
            raw: T::one() << FRAC_BITS,
        }
    }
}

impl<T: FixedBase, const FRAC_BITS: u32> Field for Fixed<T, FRAC_BITS> {
    fn div(self, rhs: Self) -> Self {
        let a = self.raw.widen();
        let b = rhs.raw.widen();
        Fixed {
            raw: T::narrow((a << FRAC_BITS).div_euclid(b)),
        }
    }
}

macro_rules! impl_fixed_base {
    ($narrow:ty, $wide:ty) => {
        impl FixedBase for $narrow {
            type Wide = $wide;
            const MIN: Self = <$narrow>::MIN;
            const MAX: Self = <$narrow>::MAX;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            fn widen(self) -> $wide {
                self as $wide
            }
            fn narrow(wide: $wide) -> $narrow {
                wide as $narrow
            }
            fn from_f64(x: f64) -> Self {
                x as $narrow
            }
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
    };
}

impl_fixed_base!(u8, u16);
impl_fixed_base!(u16, u32);
impl_fixed_base!(u32, u64);
impl_fixed_base!(u64, u128);
impl_fixed_base!(i8, i16);
impl_fixed_base!(i16, i32);
impl_fixed_base!(i32, i64);
impl_fixed_base!(i64, i128);

impl_additive_ops!([T: Additive, const FRAC_BITS: u32] Fixed<T, FRAC_BITS>);
impl_ring_ops!([T: FixedBase, const FRAC_BITS: u32] Fixed<T, FRAC_BITS>);
impl_vector_space_ops!([T: Ring, const FRAC_BITS: u32] Fixed<T, FRAC_BITS>);

#[cfg(test)]
mod tests {
    // Tier 1 — algebraic laws (must hold exactly)
    // - Additive group: associativity, commutativity, identity (zero), inverse (negate).
    // - `one()` is a true multiplicative identity: x.mult(one()) == x and x.div(one()) == x.
    // - `from_integer` is a ring homomorphism: preserves +, *, 0, 1 (within range).

    // Tier 2 — representation invariants
    // - (trunc() << FRAC_BITS) + fract() == into_raw(), with 0 <= fract() < 2^FRAC_BITS.
    // - Fixed::from_raw(x).into_raw() == x.

    // Tier 3 — approximation bounds
    // - mult truncation: |(a*b).to_f64() - a.to_f64()*b.to_f64()| <= 2^-FRAC_BITS.
    // - div truncation: similarly bounded (commit to a rounding mode for signed T).
    // - from_f64 ∘ to_f64 is identity-modulo-precision on representable values.
    // - Order preserved by to_f64.

    // Tier 4 — vector space coherence
    // - scale(c) treats c: T as an integer scalar: x.scale(T::one()) == x,
    //   x.scale(T::from_integer(2)) == x.plus(x).
}
