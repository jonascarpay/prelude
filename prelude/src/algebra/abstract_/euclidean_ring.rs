use crate::algebra::abstract_::additive::OrderedAdditive;

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

/// Greatest common divisor via the Euclidean algorithm, i.e. the intersection of their prime
/// factors.
/// Zero when both inputs are, positive otherwise.
/// `gcd(0, 0) == 0`.
/// `gcd(0, a) == a.abs()`.
///
// TODO: might be worth making a trait method, since bignums might want to override with binary gcd
pub fn gcd<T: EuclideanRing + Ord>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let r = a.clone().rem_euclid(b.clone());
        a = b;
        b = r;
    }
    a.abs()
}

/// Least common multiple via the Euclidean algorithm, i.e. the union of their prime factors.
/// Always nonnegative.
/// Zero when at least one input is, positive otherwise.
/// `lcm(0, 0) == 0`.
/// `lcm(0, a) == 0`.
pub fn lcm<T: EuclideanRing + Ord>(a: T, b: T) -> T {
    if a == T::zero() || b == T::zero() {
        return T::zero();
    }
    // Divide before multiplying to keep the intermediate small; `gcd` divides `a`
    // exactly, so the quotient is exact regardless of rounding direction.
    let g = gcd(a.clone(), b.clone());
    a.div_euclid(g).mult(b).abs()
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
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 9), 9);
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(6, 15), 3);
        assert_eq!(gcd(6, 8), 2);
        assert_eq!(gcd(12, 20), 4);
    }

    #[test]
    fn known_lcms() {
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(0, 9), 0);
        assert_eq!(lcm(6, 15), 30);
        assert_eq!(lcm(6, 8), 24);
        assert_eq!(lcm(12, 20), 60);
        assert_eq!(lcm(-4, 6), 12);
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

        #[test]
        fn lcm_commutative(a: i32, b: i32) {
            let (a, b) = (a as i64, b as i64); /* upcast to avoid overflow near boundaries */
            prop_assert_eq!(lcm(a, b), lcm(b, a));
        }

        #[test]
        fn lcm_is_nonnegative(a: i32, b: i32) {
            let (a, b) = (a as i64, b as i64); /* upcast to avoid overflow near boundaries */
            prop_assert!(lcm(a, b) >= 0);
        }

        #[test]
        fn lcm_is_multiple_of_both(a: i32, b: i32) {
            let (a, b) = (a as i64, b as i64); /* upcast to avoid overflow near boundaries */
            let l = lcm(a, b);
            if a != 0 {
                prop_assert_eq!(l % a, 0);
            }
            if b != 0 {
                prop_assert_eq!(l % b, 0);
            }
        }

        #[test]
        fn gcd_lcm_product(a: i32, b: i32) {
            let (a, b) = (a as i64, b as i64); /* upcast to avoid overflow near boundaries */
            // gcd(a, b) * lcm(a, b) == |a * b|
            prop_assert_eq!(gcd(a, b) * lcm(a, b), (a * b).abs());
        }
    }
}
