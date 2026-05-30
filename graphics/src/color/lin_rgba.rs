use crate::color::lin_rgb::LinRgb;
use prelude::{
    algebra::{
        abstract_::{Additive, Ring, VectorSpace},
        numeric::unorm8::Unorm8,
    },
    impl_additive_ops, impl_vector_space_mul,
};

/// 8-bit linear RGBA.
///
/// Note while this forms an additive group, and additive blending is valid, subtracting is _not_.
/// Subtraction is defined, but only as the inverse operation of addition.
/// Naively subtracting two colors can lead to invalid colors that violate the premultiplied
/// contraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinRgba {
    // Invariant: r, g, b <= a.
    r: Unorm8,
    g: Unorm8,
    b: Unorm8,
    a: Unorm8,
}

impl LinRgba {
    pub const TRANSPARENT: Self = Self {
        r: Unorm8::ZERO,
        g: Unorm8::ZERO,
        b: Unorm8::ZERO,
        a: Unorm8::ZERO,
    };
    pub const BLACK: Self = Self {
        r: Unorm8::ZERO,
        g: Unorm8::ZERO,
        b: Unorm8::ZERO,
        a: Unorm8::ONE,
    };
    pub const WHITE: Self = Self {
        r: Unorm8::ONE,
        g: Unorm8::ONE,
        b: Unorm8::ONE,
        a: Unorm8::ONE,
    };

    pub const fn r(self) -> Unorm8 {
        self.r
    }
    pub const fn g(self) -> Unorm8 {
        self.g
    }
    pub const fn b(self) -> Unorm8 {
        self.b
    }
    pub const fn a(self) -> Unorm8 {
        self.a
    }

    /// Returns `None` if the premultiplied invariant `r, g, b <= a` is violated.
    pub const fn from_premultiplied(r: Unorm8, g: Unorm8, b: Unorm8, a: Unorm8) -> Option<Self> {
        let c = Self { r, g, b, a };
        if c.valid_premult() {
            Some(c)
        } else {
            None
        }
    }

    /// Checks the invariant i.e. `r, g, b <= a`.
    const fn valid_premult(self) -> bool {
        self.r.0 <= self.a.0 && self.g.0 <= self.a.0 && self.b.0 <= self.a.0
    }

    /// Opaque (alpha = ONE) — premult invariant trivially holds.
    pub const fn from_rgb(rgb: LinRgb) -> Self {
        Self {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: Unorm8::ONE,
        }
    }

    /// Premultiplies `rgb` by `alpha`.
    pub fn from_rgb_transparent(rgb: LinRgb, alpha: Unorm8) -> Self {
        Self {
            r: rgb.r.mult(alpha),
            g: rgb.g.mult(alpha),
            b: rgb.b.mult(alpha),
            a: alpha,
        }
    }

    /// Porter-Duff source-over: `self` (foreground) composited on top of `dst` (background).
    /// `self + dst * (ONE - self.a)`.
    ///
    /// Forms a non-commutative monoid with TRANSPARENT as the identity (save for some potential rounding issues on the associativity)
    pub fn over(self, dst: Self) -> Self {
        let k = Unorm8::ONE.minus(self.a);
        self.plus(dst.scale(k))
    }
}

impl Additive for LinRgba {
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
        Self::TRANSPARENT
    }
    fn negate(self) -> Self {
        Self {
            r: self.r.negate(),
            g: self.g.negate(),
            b: self.b.negate(),
            a: self.a.negate(),
        }
    }
    fn is_zero(&self) -> bool {
        self.r.is_zero() && self.g.is_zero() && self.b.is_zero() && self.a.is_zero()
    }
}

impl VectorSpace for LinRgba {
    type Scalar = Unorm8;
    fn scale(self, c: Unorm8) -> Self {
        Self {
            r: self.r.mult(c),
            g: self.g.mult(c),
            b: self.b.mult(c),
            a: self.a.mult(c),
        }
    }
}

impl From<LinRgb> for LinRgba {
    fn from(c: LinRgb) -> Self {
        Self::from_rgb(c)
    }
}

impl_additive_ops!([] LinRgba);
impl_vector_space_mul!([] LinRgba);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for LinRgba {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_: ()) -> Self::Strategy {
            (any::<LinRgb>(), any::<Unorm8>())
                .prop_map(|(rgb, alpha)| LinRgba::from_rgb_transparent(rgb, alpha))
                .boxed()
        }
    }

    proptest! {
        #[test]
        fn plus_commutative(a: LinRgba, b: LinRgba) {
            prop_assert_eq!(a + b, b + a);
        }

        #[test]
        fn plus_associative(a: LinRgba, b: LinRgba, c: LinRgba) {
            prop_assert_eq!((a + b) + c, a + (b + c));
        }

        #[test]
        fn plus_identity(a: LinRgba) {
            prop_assert_eq!(a + LinRgba::TRANSPARENT, a);
        }

        #[test]
        fn scale_identity(a: LinRgba) {
            prop_assert_eq!(a * Unorm8::ONE, a);
        }

        #[test]
        fn scale_zero(a: LinRgba) {
            prop_assert_eq!(a * Unorm8::ZERO, LinRgba::TRANSPARENT);
        }

        #[test]
        fn from_rgb_transparent_premultiplied(rgb: LinRgb, alpha: Unorm8) {
            prop_assert!(LinRgba::from_rgb_transparent(rgb, alpha).valid_premult());
        }

        #[test]
        fn from_premultiplied_round_trip(a: LinRgba) {
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
        fn over_transparent_is_identity(a: LinRgba) {
            prop_assert_eq!(a.over(LinRgba::TRANSPARENT), a);
            prop_assert_eq!(LinRgba::TRANSPARENT.over(a), a);
        }

        #[test]
        fn over_opaque_foreground(rgb: LinRgb, dst: LinRgba) {
            let fg: LinRgba = rgb.into();
            prop_assert_eq!(fg.over(dst), fg);
        }

        #[test]
        fn over_preserves_premult(a: LinRgba, b: LinRgba) {
            prop_assert!(a.over(b).valid_premult());
        }

        #[test]
        fn plus_preserves_premult(a: LinRgba, b: LinRgba) {
            prop_assert!((a + b).valid_premult());
        }

        #[test]
        fn scale_preserves_premult(a: LinRgba, c: Unorm8) {
            prop_assert!((a * c).valid_premult());
        }
    }
}
