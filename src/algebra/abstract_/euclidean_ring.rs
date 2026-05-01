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

macro_rules! impl_euclidean_ring {
    ($t:ty) => {
        impl EuclideanRing for $t {
            fn div_euclid(self, rhs: Self) -> Self {
                <$t>::div_euclid(self, rhs)
            }
            fn rem_euclid(self, rhs: Self) -> Self {
                <$t>::rem_euclid(self, rhs)
            }
            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                <$t>::checked_div_euclid(self, rhs)
            }
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
