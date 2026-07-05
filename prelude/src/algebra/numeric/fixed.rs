use std::ops::{BitAnd, BitOr, BitXor, Not, Range, Shl, Shr};

use crate::{
    algebra::{
        abstract_::{
            euclidean_ring::EuclideanRing, field::Field, real::Real, Additive, Ring, VectorSpace,
        },
        numeric::fixed_base::FixedBase,
    },
    impl_additive_ops, impl_ring_ops,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fixed<T, const FRAC_BITS: u32> {
    raw: T,
}

impl<T: FixedBase + std::fmt::Debug, const FRAC_BITS: u32> std::fmt::Debug for Fixed<T, FRAC_BITS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fixed({:?} ≈ {})", self.raw, self.to_f64())
    }
}

// TODO: bit ops

impl<T, const FRAC_BITS: u32> Fixed<T, FRAC_BITS> {
    pub const fn from_raw(raw: T) -> Self {
        Fixed { raw }
    }

    pub fn repr(self) -> T {
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

    /// Construct a `Fixed` by providing a mapping of an interval.
    // TODO AsPrimitive? Save ourselves the cast?
    pub fn from_range(domain: Range<T>, range: Range<Self>, x: T) -> Self {
        // ya + (x - xa)/(xb - xa) * (yb - ya)
        // ya + (x - xa) dy / dx                # Delay the div to improve accuracy
        let dx = domain.end.minus(domain.start);
        let dy = range.end.minus(range.start).raw;
        let y = (x.minus(domain.start))
            .mult(dy)
            .div_euclid(dx)
            .plus(range.start.raw);
        Fixed { raw: y }
    }

    /// Destruct a `Fixed` by providing a mapping of an interval
    // TODO AsPrimitive? Save ourselves the cast?
    pub fn to_range(self, domain: Range<Self>, range: Range<T>) -> T {
        let dx = domain.end.minus(domain.start).raw;
        let dy = range.end.minus(range.start);
        let y = (self.minus(domain.start).raw)
            .mult(dy)
            .div_euclid(dx)
            .plus(range.start);
        y
    }

    /// Integer part. Floors toward negative infinity, such that (a.trunc() << FRAC_BITS + a.fract) = a.into_raw().
    pub fn trunc(self) -> T {
        self.raw >> FRAC_BITS
    }

    /// Fractional part, in `[0, 2^FRAC_BITS)`.
    pub fn fract(self) -> Self {
        Fixed {
            raw: self.raw & !(!(T::ZERO) << FRAC_BITS),
        }
    }
}

impl<Scalar, T: Shl<Scalar>, const FRAC_BITS: u32> Shl<Scalar> for Fixed<T, FRAC_BITS> {
    type Output = Fixed<T::Output, FRAC_BITS>;
    fn shl(self, rhs: Scalar) -> Self::Output {
        Fixed {
            raw: self.raw.shl(rhs),
        }
    }
}

impl<Scalar, T: Shr<Scalar>, const FRAC_BITS: u32> Shr<Scalar> for Fixed<T, FRAC_BITS> {
    type Output = Fixed<T::Output, FRAC_BITS>;
    fn shr(self, rhs: Scalar) -> Self::Output {
        Fixed {
            raw: self.raw.shr(rhs),
        }
    }
}

impl<T: BitAnd<T, Output = T>, const FRAC_BITS: u32> BitAnd for Fixed<T, FRAC_BITS> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Fixed {
            raw: self.raw.bitand(rhs.raw),
        }
    }
}

impl<T: BitOr<T, Output = T>, const FRAC_BITS: u32> BitOr for Fixed<T, FRAC_BITS> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Fixed {
            raw: self.raw.bitor(rhs.raw),
        }
    }
}

impl<T: BitXor<T, Output = T>, const FRAC_BITS: u32> BitXor for Fixed<T, FRAC_BITS> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Fixed {
            raw: self.raw.bitxor(rhs.raw),
        }
    }
}

impl<T: Not<Output = T>, const FRAC_BITS: u32> Not for Fixed<T, FRAC_BITS> {
    type Output = Self;
    fn not(self) -> Self {
        Fixed {
            raw: self.raw.not(),
        }
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

    fn minus(self, rhs: Self) -> Self {
        Fixed {
            raw: self.raw.minus(rhs.raw),
        }
    }
}

impl<T: FixedBase, const FRAC_BITS: u32> VectorSpace for Fixed<T, FRAC_BITS> {
    type Scalar = Self;
    fn scale(self, c: Self) -> Self {
        self.mult(c)
    }
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
    fn checked_div(self, rhs: Self) -> Option<Self> {
        let a = self.raw.widen();
        let b = rhs.raw.widen();
        Some(Fixed {
            raw: T::narrow((a << FRAC_BITS).checked_div_euclid(b)?),
        })
    }
}

impl<T: FixedBase, const FRAC_BITS: u32> Real for Fixed<T, FRAC_BITS> {
    fn from_f64(x: f64) -> Self {
        Self::from_f64(x)
    }

    fn sqrt(self) -> Self {
        Fixed {
            raw: T::narrow(T::isqrt(self.raw.widen() << FRAC_BITS)),
        }
    }

    fn cbrt(self) -> Self {
        todo!()
    }

    fn sin_tau(self) -> Self {
        todo!()
    }

    fn cos_tau(self) -> Self {
        todo!()
    }

    fn atan2(self, x: Self) -> Self {
        todo!()
    }
}

impl_additive_ops!([T: Additive, const FRAC_BITS: u32] Fixed<T, FRAC_BITS>);
impl_ring_ops!([T: FixedBase, const FRAC_BITS: u32] Fixed<T, FRAC_BITS>);

impl<T: std::ops::Mul<T, Output = T>, const FRAC_BITS: u32> std::ops::Mul<T>
    for Fixed<T, FRAC_BITS>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Fixed {
            raw: self.raw * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::abstract_::additive::EqAdditive;
    use proptest::prelude::*;

    type I = Fixed<i16, 8>;

    fn i() -> impl Strategy<Value = I> {
        any::<i16>().prop_map(I::from_raw)
    }

    // Tier 1 — algebraic laws (must hold exactly)

    proptest! {
        #[test]
        fn additive_associativity(a in i(), b in i(), c in i()) {
            prop_assert_eq!((a + b) + c, a + (b + c));
        }

        #[test]
        fn additive_commutativity(a in i(), b in i()) {
            prop_assert_eq!(a + b, b + a);
        }

        #[test]
        fn additive_identity(a in i()) {
            prop_assert_eq!(a + I::ZERO, a);
            prop_assert_eq!(I::ZERO + a, a);
        }

        #[test]
        fn additive_inverse(a in i()) {
            prop_assert_eq!(a + (-a), I::ZERO);
        }

        #[test]
        fn mult_identity(a in i()) {
            let one = I::one();
            prop_assert_eq!(a.mult(one), a);
            prop_assert_eq!(one.mult(a), a);
        }

        #[test]
        fn div_identity(a in i()) {
            prop_assert_eq!(a.div(I::one()), a);
        }

        #[test]
        fn from_integer_homomorphism(i in -10isize..=10, j in -10isize..=10) {
            prop_assert_eq!(
                I::from_integer(i + j),
                I::from_integer(i) + I::from_integer(j),
            );
            prop_assert_eq!(
                I::from_integer(i * j),
                I::from_integer(i).mult(I::from_integer(j)),
            );
        }
    }

    #[test]
    fn from_integer_zero_one() {
        assert_eq!(I::from_integer(0), I::ZERO);
        assert_eq!(I::from_integer(1), I::one());
    }

    // Tier 2 — representation invariants

    proptest! {
        #[test]
        fn raw_roundtrip(x in any::<i16>()) {
            prop_assert_eq!(I::from_raw(x).repr(), x);
        }

        #[test]
        fn trunc_fract_decomposition(a in i()) {
            let t = a.trunc();
            let f = a.fract();
            prop_assert_eq!((t << 8) + f.repr(), a.repr());
            prop_assert!((0..256).contains(&f.repr()));
        }
    }

    // Multiplication and division — algebraic laws and their failure modes.

    fn approx(a: I, b: I, epsilon: i32) -> bool {
        (a.repr() as i32 - b.repr() as i32).abs() <= epsilon
    }

    /// Values with real magnitude ≤ 1 (raw in [-256, 256]) — small enough that
    /// triple products and pairwise sums never overflow i16 or the wide i32 path.
    fn small_i() -> impl Strategy<Value = I> {
        (-256i16..=256).prop_map(I::from_raw)
    }

    /// Values with real magnitude ≥ 1 (raw |β| ≥ 256). Keeps `a/b` representable when |a| ≤ 1.
    fn at_least_one() -> impl Strategy<Value = I> {
        prop_oneof![
            (i16::MIN..=-256i16).prop_map(I::from_raw),
            (256i16..=i16::MAX).prop_map(I::from_raw),
        ]
    }

    proptest! {
        // Exact laws

        #[test]
        fn mult_commutative(a in i(), b in i()) {
            prop_assert_eq!(a.mult(b), b.mult(a));
        }

        #[test]
        fn mult_annihilator(a in i()) {
            prop_assert_eq!(a.mult(I::ZERO), I::ZERO);
        }

        #[test]
        fn self_div(a in i().prop_filter("nonzero", |a| !a.is_zero())) {
            prop_assert_eq!(a.div(a), I::one());
        }

        // TODO clean all this up

        // Approximate laws — each mult truncates by ≤ 1 raw unit (floor toward −∞ on signed shift).

        /// (a*b)*c vs a*(b*c): each side accumulates 2 mult-truncations; on |x| ≤ 1
        /// inputs the propagated error stays bounded by 2 ULP.
        #[test]
        fn mult_associative(a in small_i(), b in small_i(), c in small_i()) {
            let lhs = a.mult(b).mult(c);
            let rhs = a.mult(b.mult(c));
            prop_assert!(approx(lhs, rhs, 2), "{:?} vs {:?}", lhs, rhs);
        }

        /// a*(b+c) vs a*b + a*c: LHS has one truncation, RHS has two but `(b+c)` is exact;
        /// difference is the carry from floor-non-additivity, ≤ 1 ULP.
        #[test]
        fn mult_distributive(a in small_i(), b in small_i(), c in small_i()) {
            let lhs = a.mult(b + c);
            let rhs = a.mult(b) + a.mult(c);
            prop_assert!(approx(lhs, rhs, 1), "{:?} vs {:?}", lhs, rhs);
        }

        /// (a/b)*b ≈ a, with |a| ≤ 1 and |b| ≥ 1. Division remainder r ∈ [0, |β|);
        /// multiplying back loses floor(r/256) ULP, so error ≤ ⌈|β|/256⌉.
        #[test]
        fn div_mult_roundtrip(a in small_i(), b in at_least_one()) {
            let r = a.div(b).mult(b);
            let bound = (b.repr().unsigned_abs() as i32 + 255) / 256;
            prop_assert!(approx(r, a, bound), "{:?} vs {:?} (bound {})", r, a, bound);
        }
    }

    #[test]
    fn from_range_endpoints() {
        type R = Fixed<i32, 8>;
        let xa = -234;
        let xb = 567;
        let ya = R::from_f64(-3.14);
        let yb = R::from_f64(7.54);
        assert_eq!(R::from_range(xa..xb, ya..yb, xa), ya);
        assert_eq!(R::from_range(xa..xb, ya..yb, xb), yb);
    }

    #[test]
    fn to_range_endpoints() {
        type R = Fixed<i32, 8>;
        let xa = R::from_f64(-3.14);
        let xb = R::from_f64(7.54);
        let ya = -234i32;
        let yb = 567;
        assert_eq!(xa.to_range(xa..xb, ya..yb), ya);
        assert_eq!(xb.to_range(xa..xb, ya..yb), yb);
    }
}
