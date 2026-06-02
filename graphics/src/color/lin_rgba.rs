use crate::color::lin_rgb::LinRgb;
use prelude::{
    algebra::{
        abstract_::{Additive, Ring, VectorSpace},
        numeric::unorm8::Unorm8,
    },
    impl_additive_ops, impl_vector_space_mul,
};

/// Linear RGBA, generic over the per-channel pixel type `T`.
///
/// Note while this forms an additive group, and additive blending is valid, subtracting is _not_.
/// Subtraction is defined, but only as the inverse operation of addition.
/// Naively subtracting two colors can lead to invalid colors that violate the premultiplied
/// contraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinRgba<T> {
    // Invariant: r, g, b <= a.
    r: T,
    g: T,
    b: T,
    a: T,
}

/// 8-bit linear RGBA — the storage/compositing representation.
pub type LinRgba8 = LinRgba<Unorm8>;

impl<T: Copy> LinRgba<T> {
    pub fn r(self) -> T {
        self.r
    }
    pub fn g(self) -> T {
        self.g
    }
    pub fn b(self) -> T {
        self.b
    }
    pub fn a(self) -> T {
        self.a
    }
}

impl<T: PartialOrd + Copy> LinRgba<T> {
    /// Returns `None` if the premultiplied invariant `r, g, b <= a` is violated.
    pub fn from_premultiplied(r: T, g: T, b: T, a: T) -> Option<Self> {
        let c = Self { r, g, b, a };
        if c.valid_premult() {
            Some(c)
        } else {
            None
        }
    }

    /// Checks the invariant i.e. `r, g, b <= a`.
    fn valid_premult(self) -> bool {
        self.r <= self.a && self.g <= self.a && self.b <= self.a
    }
}

impl<T: Ring> LinRgba<T> {
    pub fn transparent() -> Self {
        Self {
            r: T::zero(),
            g: T::zero(),
            b: T::zero(),
            a: T::zero(),
        }
    }
    pub fn black() -> Self {
        Self {
            r: T::zero(),
            g: T::zero(),
            b: T::zero(),
            a: T::one(),
        }
    }
    pub fn white() -> Self {
        Self {
            r: T::one(),
            g: T::one(),
            b: T::one(),
            a: T::one(),
        }
    }

    /// Opaque (alpha = one) — premult invariant trivially holds.
    pub fn from_rgb(rgb: LinRgb<T>) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: T::one(),
        }
    }

    /// Premultiplies `rgb` by `alpha`.
    pub fn from_rgb_transparent(rgb: LinRgb<T>, alpha: T) -> Self {
        Self {
            r: rgb.r.mult(alpha.clone()),
            g: rgb.g.mult(alpha.clone()),
            b: rgb.b.mult(alpha.clone()),
            a: alpha,
        }
    }

    /// Porter-Duff source-over: `self` (foreground) composited on top of `dst` (background).
    /// `self + dst * (one - self.a)`.
    ///
    /// Forms a non-commutative monoid with `transparent()` as the identity (save for some potential rounding issues on the associativity)
    pub fn over(self, dst: Self) -> Self {
        let k = T::one().minus(self.a.clone());
        self.plus(dst.scale(k))
    }
}

impl<T: Additive> Additive for LinRgba<T> {
    fn plus(self, rhs: Self) -> Self {
        Self {
            r: self.r.plus(rhs.r),
            g: self.g.plus(rhs.g),
            b: self.b.plus(rhs.b),
            a: self.a.plus(rhs.a),
        }
    }
    fn minus(self, rhs: Self) -> Self {
        Self {
            r: self.r.minus(rhs.r),
            g: self.g.minus(rhs.g),
            b: self.b.minus(rhs.b),
            a: self.a.minus(rhs.a),
        }
    }
    fn zero() -> Self {
        Self {
            r: T::zero(),
            g: T::zero(),
            b: T::zero(),
            a: T::zero(),
        }
    }
    fn negate(self) -> Self {
        Self {
            r: self.r.negate(),
            g: self.g.negate(),
            b: self.b.negate(),
            a: self.a.negate(),
        }
    }
}

impl<T: Ring> VectorSpace for LinRgba<T> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Self {
            r: self.r.mult(c.clone()),
            g: self.g.mult(c.clone()),
            b: self.b.mult(c.clone()),
            a: self.a.mult(c),
        }
    }
}

impl<T: Ring> From<LinRgb<T>> for LinRgba<T> {
    fn from(c: LinRgb<T>) -> Self {
        Self::from_rgb(c)
    }
}

impl_additive_ops!([T: Additive] LinRgba<T>);
impl_vector_space_mul!([T: Ring] LinRgba<T>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::lin_rgb::LinRgb8;
    use proptest::prelude::*;

    impl<T: Arbitrary + Ring + 'static> Arbitrary for LinRgba<T> {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_: ()) -> Self::Strategy {
            (any::<LinRgb<T>>(), any::<T>())
                .prop_map(|(rgb, alpha)| LinRgba::from_rgb_transparent(rgb, alpha))
                .boxed()
        }
    }

    proptest! {
        // Explicitly testing LinRgba8 here
        #[test]
        fn plus_commutative(a: LinRgba8, b: LinRgba8) {
            prop_assert_eq!(a + b, b + a);
        }

        #[test]
        fn plus_associative(a: LinRgba8, b: LinRgba8, c: LinRgba8) {
            prop_assert_eq!((a + b) + c, a + (b + c));
        }

        #[test]
        fn plus_identity(a: LinRgba8) {
            prop_assert_eq!(a + LinRgba::transparent(), a);
        }

        #[test]
        fn scale_identity(a: LinRgba8) {
            prop_assert_eq!(a * Unorm8::ONE, a);
        }

        #[test]
        fn scale_zero(a: LinRgba8) {
            prop_assert_eq!(a * Unorm8::ZERO, LinRgba::transparent());
        }

        #[test]
        fn from_rgb_transparent_premultiplied(rgb: LinRgb8, alpha: Unorm8) {
            prop_assert!(LinRgba::from_rgb_transparent(rgb, alpha).valid_premult());
        }

        #[test]
        fn from_premultiplied_round_trip(a: LinRgba8) {
            prop_assert_eq!(
                LinRgba::from_premultiplied(a.r(), a.g(), a.b(), a.a()),
                Some(a),
            );
        }

        #[test]
        fn from_premultiplied_rejects_invalid(r: Unorm8, g: Unorm8, b: Unorm8, a: Unorm8) {
            let invalid = r.0 > a.0 || g.0 > a.0 || b.0 > a.0;
            prop_assert_eq!(
                LinRgba::from_premultiplied(r, g, b, a).is_none(),
                invalid,
            );
        }

        #[test]
        fn over_transparent_is_identity(a: LinRgba8) {
            prop_assert_eq!(a.over(LinRgba::transparent()), a);
            prop_assert_eq!(LinRgba::transparent().over(a), a);
        }

        #[test]
        fn over_opaque_foreground(rgb: LinRgb8, dst: LinRgba8) {
            let fg: LinRgba8 = rgb.into();
            prop_assert_eq!(fg.over(dst), fg);
        }

        #[test]
        fn over_preserves_premult(a: LinRgba8, b: LinRgba8) {
            prop_assert!(a.over(b).valid_premult());
        }

        #[test]
        fn plus_preserves_premult(a: LinRgba8, b: LinRgba8) {
            prop_assert!((a + b).valid_premult());
        }

        #[test]
        fn scale_preserves_premult(a: LinRgba8, c: Unorm8) {
            prop_assert!((a * c).valid_premult());
        }
    }
}
