use std::ops::{BitAnd, Not, Shl, Shr};

use crate::algebra::abstract_::EuclideanRing;

// TODO trait synonym
pub trait FixedBase:
    Copy
    + EuclideanRing
    + Not<Output = Self>
    + BitAnd<Self, Output = Self>
    + Eq
    + Shl<u32, Output = Self>
    + Shl<i32, Output = Self>
    + Shr<u32, Output = Self>
    + Shr<i32, Output = Self>
{
    type Wide: EuclideanRing + Shl<u32, Output = Self::Wide> + Shr<u32, Output = Self::Wide>;
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self; // TODO from Additive
    const ONE: Self; // TODO from Ring
    const BITS: u32; // TODO unused
    fn widen(self) -> Self::Wide;
    fn narrow(wide: Self::Wide) -> Self;
    fn leading_zeros(self) -> u32; // TODO unused, remove
    fn trailing_zeros(self) -> u32; // TODO unused, remove
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    fn checked_shr(self, rhs: u32) -> Option<Self>;
    fn from_f64(x: f64) -> Self;
    fn to_f64(self) -> f64;
    // May be moved to a trait at some point
    fn isqrt(wide: Self::Wide) -> Self::Wide;
}

macro_rules! impl_fixed_base {
    ($narrow:ty, $wide:ty) => {
        impl FixedBase for $narrow {
            type Wide = $wide;
            const MIN: Self = <$narrow>::MIN;
            const MAX: Self = <$narrow>::MAX;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const BITS: u32 = <$narrow>::BITS;
            fn leading_zeros(self) -> u32 {
                self.leading_zeros()
            }
            fn trailing_zeros(self) -> u32 {
                self.trailing_zeros()
            }
            fn checked_shl(self, rhs: u32) -> Option<Self> {
                <$narrow>::checked_shl(self, rhs)
            }
            fn checked_shr(self, rhs: u32) -> Option<Self> {
                <$narrow>::checked_shr(self, rhs)
            }
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
            fn isqrt(wide: $wide) -> $wide {
                wide.isqrt()
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
