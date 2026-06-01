use prelude::algebra::numeric::unorm8::Unorm8;

use crate::color::lin_rgb::LinRgb8;

/// 8-bit gamma-2.2-encoded sRGB. No alpha.
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
    pub const fn encode_softbuffer(self) -> u32 {
        (self.r.0 as u32) << 16 | (self.g.0 as u32) << 8 | (self.b.0 as u32)
    }
}

impl From<LinRgb8> for Srgb {
    fn from(c: LinRgb8) -> Self {
        Self {
            r: c.r.to_gamma(),
            g: c.g.to_gamma(),
            b: c.b.to_gamma(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::algebra::abstract_::Additive;

    #[test]
    fn black_white() {
        assert_eq!(Srgb::from(LinRgb8::zero()), Srgb::BLACK);
        assert_eq!(Srgb::from(LinRgb8::white()), Srgb::WHITE);
    }
}
