use super::ring::Ring;

/// A (commutative) ring with a division-with-remainder operation.
///
/// For all `a` and `b != zero()`, satisfies
///   a == a.div_euclid(b).mult(b).plus(a.rem_euclid(b))
/// and `a.rem_euclid(b) < b` for an implicit size function (typically just abs)
// TODO polynomials are euclidean rings, but I haven't figured out yet exactly how or why
pub trait EuclideanRing: Ring + Sized {
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;
}

/// Greatest common divisor via the Euclidean algorithm.
/// `gcd(0, 0) == 0`.
///
// TODO: might be worth making a trait method, since bignums might want to override with binary gcd
pub fn gcd<T: EuclideanRing + Ord>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let r = a.clone().rem_euclid(b.clone());
        a = b;
        b = r;
    }
    if a < T::zero() {
        a.negate()
    } else {
        a
    }
}

macro_rules! impl_euclidean_ring {
    ($t:ty) => {
        impl EuclideanRing for $t {
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                <$t>::div_euclid(self, rhs)
            }
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                <$t>::rem_euclid(self, rhs)
            }
            #[inline]
            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                <$t>::checked_div_euclid(self, rhs)
            }
            #[inline]
            fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                <$t>::checked_rem_euclid(self, rhs)
            }
        }
    };
}

impl_euclidean_ring!(i8);
impl_euclidean_ring!(i16);
impl_euclidean_ring!(i32);
impl_euclidean_ring!(i64);
impl_euclidean_ring!(i128);
impl_euclidean_ring!(isize);
impl_euclidean_ring!(u8);
impl_euclidean_ring!(u16);
impl_euclidean_ring!(u32);
impl_euclidean_ring!(u64);
impl_euclidean_ring!(u128);
impl_euclidean_ring!(usize);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn known_gcds() {
        // assert_eq!(gcd(0, 0), 0);
        // assert_eq!(gcd(0, 9), 1);
        // assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(6, 15), 3);
        assert_eq!(gcd(6, 8), 2);
        assert_eq!(gcd(12, 20), 4);
    }

    proptest! {

        #[test]
        fn gcd_associative(a: i32, b: i32) {
            let (a, b) = (a as i64, b as i64); /* upcast to avoid underflow near boundaries */
            prop_assert_eq!(gcd(a,b), gcd(b,a));
        }

        #[test]
        fn gcd_divides_both(a: i32, b: i32) {
            let (a, b) = (a as i64, b as i64); /* upcast to avoid underflow near boundaries */
            let g = gcd(a, b);
            prop_assert!(g >= 0);
            if a != 0 {
                prop_assert_eq!(a % g, 0);
            }
            if b != 0 {
                prop_assert_eq!(b % g, 0);
            }
        }

        #[test]
        fn gcd_is_positive(a: i32, b: i32) {
            let (a, b) = (a as i64, b as i64); /* upcast to avoid underflow near boundaries */
            prop_assert!((a == 0 && b == 0) || gcd(a, b) > 0);
        }

        #[test]
        fn gcd_zero_is_abs(a: i32) {
            prop_assert_eq!(gcd(0, a), a.abs());
        }
    }
}
