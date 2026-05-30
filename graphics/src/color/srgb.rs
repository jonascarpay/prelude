use crate::color::lin_rgb::LinRgb;
use crate::color::unorm8::Unorm8;

/// 8-bit gamma-2.0-encoded sRGB. No alpha.
///
/// Output-only — arithmetic in gamma space is incorrect, so this type has no
/// algebra impls. Render in `LinRgb` / `LinRgba`, convert here at the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Srgb {
    pub r: Unorm8,
    pub g: Unorm8,
    pub b: Unorm8,
}

impl Srgb {
    pub const BLACK: Self = Self {
        r: Unorm8::ZERO,
        g: Unorm8::ZERO,
        b: Unorm8::ZERO,
    };
    pub const WHITE: Self = Self {
        r: Unorm8::ONE,
        g: Unorm8::ONE,
        b: Unorm8::ONE,
    };
}

impl From<LinRgb> for Srgb {
    fn from(c: LinRgb) -> Self {
        Self {
            r: encode(c.r),
            g: encode(c.g),
            b: encode(c.b),
        }
    }
}

/// Gamma 2.0 encode: `srgb / 255 = sqrt(linear / 255)` => `srgb = isqrt(255 * linear)`.
fn encode(linear: Unorm8) -> Unorm8 {
    Unorm8((255u16 * linear.0 as u16).isqrt() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn encode_monotonic(a: Unorm8, b: Unorm8) {
            let lo = a.min(b);
            let hi = a.max(b);
            prop_assert!(encode(lo) <= encode(hi));
        }

        /// For every linear value, `encode(x)^2` is within 1 of `255 * x` — the
        /// best possible u8 approximation of `sqrt(255 * x)`.
        #[test]
        fn encode_is_best_isqrt(x: Unorm8) {
            let s = encode(x).0 as u32;
            let target = 255 * x.0 as u32;
            prop_assert!(s * s <= target);
            prop_assert!((s + 1) * (s + 1) > target);
        }
    }

    #[test]
    fn encode_endpoints() {
        assert_eq!(encode(Unorm8::ZERO), Unorm8::ZERO);
        assert_eq!(encode(Unorm8::ONE), Unorm8::ONE);
    }

    #[test]
    fn black_white() {
        assert_eq!(Srgb::from(LinRgb::BLACK), Srgb::BLACK);
        assert_eq!(Srgb::from(LinRgb::WHITE), Srgb::WHITE);
    }
}
