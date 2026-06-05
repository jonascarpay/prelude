use std::ops::{Shl, Shr};

use crate::algebra::{abstract_::EuclideanRing, Ring};

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
