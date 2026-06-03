use crate::{
    algebra::{
        abstract_::{
            additive::{EqAdditive, OrderedAdditive},
            euclidean_ring::gcd,
            field::Field,
            Additive, EuclideanRing,
        },
        Ring,
    },
    extra::BoolExt,
    impl_additive_ops, impl_field_ops, impl_ring_ops,
};
use std::cmp::Ordering;

pub fn ratio<T: Ord + EuclideanRing>(p: T, q: T) -> Option<Ratio<T>> {
    Ratio::new(p, q)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ratio<T> {
    /// The numerator. The sign lives here, since the denominator is always positive.
    p: T,
    /// The denominator. Always positive, and coprime to the numerator.
    q: T,
}

impl<T> Ratio<T> {
    /// The numerator. The sign lives here, since the denominator is always positive.
    pub fn numer(&self) -> &T {
        &self.p
    }

    /// The denominator. Always positive, and coprime to the numerator.
    pub fn denom(&self) -> &T {
        &self.q
    }
}

impl<T: EuclideanRing + Ord> Additive for Ratio<T> {
    fn plus(self, rhs: Self) -> Self {
        // (pa / qa) + (pb / qb), combined over lcm(qa, qb) rather than qa*qb so
        // the intermediate denominator stays as small as possible.
        //   lcm = qa/g * qb,  p = pa*(qb/g) + pb*(qa/g)   where g = gcd(qa, qb)
        let Ratio { p: pa, q: qa } = self;
        let Ratio { p: pb, q: qb } = rhs;
        let g = gcd(qa.clone(), qb.clone());
        let lcm = qa.clone().div_euclid(g.clone()).mult(qb.clone());
        let p = pa
            .mult(qb.div_euclid(g.clone()))
            .plus(pb.mult(qa.div_euclid(g)));
        // p and lcm can still share a factor (dividing g), so a final reduce is
        // needed — but it operates on lcm, not the full qa*qb product.
        reduce(p, lcm)
    }

    fn zero() -> Self {
        Ratio {
            p: T::zero(),
            q: T::one(),
        }
    }

    fn negate(self) -> Self {
        Ratio {
            p: self.p.negate(),
            q: self.q,
        }
    }
}

impl<T: EuclideanRing + Ord> Ring for Ratio<T> {
    fn mult(self, rhs: Self) -> Self {
        // (pa / qa) * (pb / qb) = pa pb / qa qb,
        // but we cancel the diagonals *before* multiplying.
        let Ratio { p: pa, q: qa } = self;
        let Ratio { p: pb, q: qb } = rhs;
        let g1 = gcd(pa.clone(), qb.clone());
        let g2 = gcd(pb.clone(), qa.clone());
        let p = pa.div_euclid(g1.clone()).mult(pb.div_euclid(g2.clone()));
        let q = qa.div_euclid(g2).mult(qb.div_euclid(g1));
        Ratio { p, q }
    }

    fn from_integer(i: isize) -> Self {
        Ratio {
            p: T::from_integer(i),
            q: T::one(),
        }
    }
    fn one() -> Self {
        Self {
            p: T::one(),
            q: T::one(),
        }
    }
}

impl<T: EuclideanRing + Ord> Field for Ratio<T> {
    fn recip(self) -> Self {
        // (q / p), then push the sign into the numerator to keep q > 0.
        // Coprimality is preserved by the swap, so no reduce is needed.
        debug_assert!(self.p.is_nonzero(), "recip of zero");
        let Ratio { p, q } = self;
        if p.is_negative() {
            Ratio {
                p: q.negate(),
                q: p.negate(),
            }
        } else {
            Ratio { p: q, q: p }
        }
    }

    fn div(self, rhs: Self) -> Self {
        self.mult(rhs.recip())
    }

    fn checked_recip(self) -> Option<Self> {
        self.p.is_nonzero().ok()?;
        Some(self.recip())
    }

    fn checked_div(self, rhs: Self) -> Option<Self> {
        Some(self.mult(rhs.checked_recip()?))
    }
}

impl_additive_ops!([T: EuclideanRing + Ord] Ratio<T>);
impl_ring_ops!([T: EuclideanRing + Ord] Ratio<T>);
impl_field_ops!([T: EuclideanRing + Ord] Ratio<T>);

impl<T: EuclideanRing + Ord> Ord for Ratio<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Both denominators are positive, so cross-multiplication preserves
        // the sign of the comparison: pa/qa <=> pb/qb  iff  pa*qb <=> pb*qa.
        let lhs = self.p.clone().mult(other.q.clone());
        let rhs = other.p.clone().mult(self.q.clone());
        lhs.cmp(&rhs)
    }
}

impl<T: EuclideanRing + Ord> PartialOrd for Ratio<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn reduce<T: Ord + EuclideanRing>(p: T, q: T) -> Ratio<T> {
    debug_assert!(q.is_positive());
    let d = gcd(p.clone(), q.clone());
    Ratio {
        p: p.div_euclid(d.clone()),
        q: q.div_euclid(d),
    }
}

impl<T: Ord + Additive + EuclideanRing> Ratio<T> {
    pub fn new(p: T, q: T) -> Option<Self> {
        Ratio { p, q }.canonical()
    }

    pub fn floor(self) -> T {
        let Ratio { p, q } = self;
        p.div_euclid(q)
    }

    pub fn ceil(self) -> T {
        let Ratio { p, q } = self;
        p.negate().div_euclid(q).negate()
    }

    /// Round towards zero.
    pub fn trunc(self) -> T {
        if self.p.is_negative() {
            self.ceil()
        } else {
            self.floor()
        }
    }

    pub fn round(self) -> T {
        let Ratio { p, q } = self;
        let floor = p.clone().div_euclid(q.clone());
        let rem = p.rem_euclid(q.clone());
        if rem.clone().plus(rem) > q {
            floor.succ()
        } else {
            floor
        }
    }

    fn canonical(self) -> Option<Self> {
        let Ratio { p, q } = self;
        q.is_nonzero().ok()?;
        let d = gcd(p.clone(), q.clone());
        let (p, q) = (p.div_euclid(d.clone()), q.clone().div_euclid(d));
        let (p, q) = if q.is_negative() {
            (p.negated(), q.negated())
        } else {
            (p, q)
        };
        Some(Ratio { p, q })
    }
}

#[cfg(any(test, feature = "proptest"))]
pub mod proptest_impls {
    use super::*;
    use proptest::prelude::*;

    type R = Ratio<i64>;

    pub fn gen_ratio() -> impl Strategy<Value = R> {
        (-100i64..=100, 1i64..=100).prop_map(|(p, q)| ratio(p, q).unwrap())
    }

    pub fn gen_nonzero_ratio() -> impl Strategy<Value = R> {
        (prop_oneof![-1000i64..=-1, 1i64..=1000], 1i64..=1000)
            .prop_map(|(p, q)| ratio(p, q).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::proptest_impls::*;
    use super::*;
    use proptest::prelude::*;

    type R = Ratio<i64>;

    fn is_canonical(r: &R) -> bool {
        r.q > 0 && gcd(r.p, r.q) == 1
    }

    proptest! {
        // Additive
        #[test]
        fn add_associative(a in gen_ratio(), b in gen_ratio(), c in gen_ratio()) {
            prop_assert_eq!(a.plus(b).plus(c), a.plus(b.plus(c)));
        }
        #[test]
        fn add_commutative(a in gen_ratio(), b in gen_ratio()) {
            prop_assert_eq!(a.plus(b), b.plus(a));
        }
        #[test]
        fn add_identity(a in gen_ratio()) {
            prop_assert_eq!(a.plus(R::zero()), a);
        }
        #[test]
        fn add_inverse(a in gen_ratio()) {
            prop_assert_eq!(a.plus(a.negate()), R::zero());
        }

        // Ring
        #[test]
        fn mul_associative(a in gen_ratio(), b in gen_ratio(), c in gen_ratio()) {
            prop_assert_eq!(a.mult(b).mult(c), a.mult(b.mult(c)));
        }
        #[test]
        fn mul_commutative(a in gen_ratio(), b in gen_ratio()) {
            prop_assert_eq!(a.mult(b), b.mult(a));
        }
        #[test]
        fn mul_identity(a in gen_ratio()) {
            prop_assert_eq!(a.mult(R::one()), a);
        }
        #[test]
        fn mul_inverse(a in gen_nonzero_ratio()) {
            prop_assert_eq!(a.mult(a.recip()), R::one());
        }

        #[test]
        fn distributive(a in gen_ratio(), b in gen_ratio(), c in gen_ratio()) {
            prop_assert_eq!(a.mult(b.plus(c)), a.mult(b).plus(a.mult(c)));
        }

        // Field
        #[test]
        fn recip_involutive(a in gen_nonzero_ratio()) {
            prop_assert_eq!(a.recip().recip(), a);
        }
        #[test]
        fn div_is_mul_recip(a in gen_ratio(), b in gen_nonzero_ratio()) {
            prop_assert_eq!(a.div(b), a.mult(b.recip()));
        }
        #[test]
        fn self_div_is_one(a in gen_nonzero_ratio()) {
            prop_assert_eq!(a.div(a), R::one());
        }
        #[test]
        fn checked_div_matches_div(a in gen_ratio(), b in gen_nonzero_ratio()) {
            prop_assert_eq!(a.checked_div(b), Some(a.div(b)));
        }
        #[test]
        fn checked_div_by_zero_is_none(a in gen_ratio()) {
            prop_assert_eq!(a.checked_div(R::zero()), None);
        }

        // Canonical
        #[test]
        fn is_canonical_is_canonical_identity(p: i8, q: i8) {
            let r = Ratio {p: p as i64, q: q as i64};
            prop_assert_eq!(
              r.canonical() == Some(r),
              is_canonical(&r)
            )
        }
        #[test]
        fn new_canonical(p in -1000i64..=1000, q in -1000i64..=1000) {
            let r = Ratio::new(p, q);
            prop_assert!(r.is_none_or(|r| is_canonical(&r)));
        }
        #[test]
        fn add_canonical(a in gen_ratio(), b in gen_ratio()) {
            prop_assert!(is_canonical(&a.plus(b)));
        }
        #[test]
        fn mult_canonical(a in gen_ratio(), b in gen_ratio()) {
            prop_assert!(is_canonical(&a.mult(b)));
        }
        #[test]
        fn recip_canonical(a in gen_nonzero_ratio()) {
            prop_assert!(is_canonical(&a.recip()));
        }
        #[test]
        fn div_canonical(a in gen_ratio(), b in gen_nonzero_ratio()) {
            prop_assert!(is_canonical(&a.div(b)));
        }

        #[test]
        fn ord_matches_f64(a in gen_ratio(), b in gen_ratio()) {
            prop_assert_eq!(a.cmp(&b), as_f64(a).partial_cmp(&as_f64(b)).unwrap());
        }

        #[test]
        fn rounding_matches_f64(a in gen_ratio()) {
            let x = as_f64(a);
            prop_assert_eq!(a.floor(), x.floor() as i64);
            prop_assert_eq!(a.ceil(), x.ceil() as i64);
            prop_assert_eq!(a.trunc(), x.trunc() as i64);
            // Unlike `f64::round`, `round` rounds halves down
        }
    }

    fn as_f64(r: R) -> f64 {
        r.p as f64 / r.q as f64
    }

    #[test]
    fn checked_recip_of_zero_is_none() {
        assert_eq!(R::zero().checked_recip(), None);
    }

    #[test]
    fn constants_canonical() {
        assert!(is_canonical(&R::zero()));
        assert!(is_canonical(&R::one()));
    }
}
