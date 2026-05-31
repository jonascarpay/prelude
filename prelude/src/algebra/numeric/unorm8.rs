use crate::algebra::abstract_::{Additive, Ring};
use crate::{impl_additive_ops, impl_ring_ops};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A real on the [0,1] interval with 8 bits of precision.
/// Addition saturates.
pub struct Unorm8(pub u8);
// TODO make generic over T using FixedBase
// TODO move to prelude, generally useful not just for color

impl Unorm8 {
    pub const ZERO: Unorm8 = Unorm8(0u8);
    pub const ONE: Unorm8 = Unorm8(u8::MAX);
    pub const EPSILON: Unorm8 = Unorm8(1u8);
    pub fn mult_blinn(self, rhs: Self) -> Self {
        let xy = self.0 as u16 * rhs.0 as u16 + 128;
        Unorm8(((xy + (xy >> 8)) >> 8) as u8)
    }
    pub fn mult_ref(self, rhs: Self) -> Self {
        let xy = self.0 as u16 * rhs.0 as u16;
        Unorm8(((xy + 127) / 255) as u8)
    }
    // Encode from linear space to gamma (display) space
    pub fn to_gamma(self) -> Self {
        Unorm8(ENCODE_LUT[self.0 as usize])
    }
    // Decode from gamma (display) to linear space
    pub fn from_gamma(self) -> Self {
        Unorm8(DECODE_LUT[self.0 as usize])
    }
    // TODO: from ratio(nal)
}

#[cfg(test)]
const GAMMA: f64 = 2.2;

#[rustfmt::skip]
const ENCODE_LUT: [u8; 256] = [
      0,  21,  28,  34,  39,  43,  46,  50,  53,  56,  59,  61,  64,  66,  68,  70,
     72,  74,  76,  78,  80,  82,  84,  85,  87,  89,  90,  92,  93,  95,  96,  98,
     99, 101, 102, 103, 105, 106, 107, 109, 110, 111, 112, 114, 115, 116, 117, 118,
    119, 120, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 144, 145, 146, 147, 148, 149, 150,
    151, 151, 152, 153, 154, 155, 156, 156, 157, 158, 159, 160, 160, 161, 162, 163,
    164, 164, 165, 166, 167, 167, 168, 169, 170, 170, 171, 172, 173, 173, 174, 175,
    175, 176, 177, 178, 178, 179, 180, 180, 181, 182, 182, 183, 184, 184, 185, 186,
    186, 187, 188, 188, 189, 190, 190, 191, 192, 192, 193, 194, 194, 195, 195, 196,
    197, 197, 198, 199, 199, 200, 200, 201, 202, 202, 203, 203, 204, 205, 205, 206,
    206, 207, 207, 208, 209, 209, 210, 210, 211, 212, 212, 213, 213, 214, 214, 215,
    215, 216, 217, 217, 218, 218, 219, 219, 220, 220, 221, 221, 222, 223, 223, 224,
    224, 225, 225, 226, 226, 227, 227, 228, 228, 229, 229, 230, 230, 231, 231, 232,
    232, 233, 233, 234, 234, 235, 235, 236, 236, 237, 237, 238, 238, 239, 239, 240,
    240, 241, 241, 242, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 248,
    248, 249, 249, 249, 250, 250, 251, 251, 252, 252, 253, 253, 254, 254, 255, 255,
];

#[rustfmt::skip]
const DECODE_LUT: [u8; 256] = [
      0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   1,
      1,   1,   1,   1,   1,   1,   1,   1,   1,   2,   2,   2,   2,   2,   2,   2,
      3,   3,   3,   3,   3,   4,   4,   4,   4,   5,   5,   5,   5,   6,   6,   6,
      6,   7,   7,   7,   8,   8,   8,   9,   9,   9,  10,  10,  11,  11,  11,  12,
     12,  13,  13,  13,  14,  14,  15,  15,  16,  16,  17,  17,  18,  18,  19,  19,
     20,  20,  21,  22,  22,  23,  23,  24,  25,  25,  26,  26,  27,  28,  28,  29,
     30,  30,  31,  32,  33,  33,  34,  35,  35,  36,  37,  38,  39,  39,  40,  41,
     42,  43,  43,  44,  45,  46,  47,  48,  49,  49,  50,  51,  52,  53,  54,  55,
     56,  57,  58,  59,  60,  61,  62,  63,  64,  65,  66,  67,  68,  69,  70,  71,
     73,  74,  75,  76,  77,  78,  79,  81,  82,  83,  84,  85,  87,  88,  89,  90,
     91,  93,  94,  95,  97,  98,  99, 100, 102, 103, 105, 106, 107, 109, 110, 111,
    113, 114, 116, 117, 119, 120, 121, 123, 124, 126, 127, 129, 130, 132, 133, 135,
    137, 138, 140, 141, 143, 145, 146, 148, 149, 151, 153, 154, 156, 158, 159, 161,
    163, 165, 166, 168, 170, 172, 173, 175, 177, 179, 181, 182, 184, 186, 188, 190,
    192, 194, 196, 197, 199, 201, 203, 205, 207, 209, 211, 213, 215, 217, 219, 221,
    223, 225, 227, 229, 231, 234, 236, 238, 240, 242, 244, 246, 248, 251, 253, 255,
];

/// Saturating addition.
///
/// Unlawful in the following ways:
/// - `a.negate()` is always 0.
/// - `a - b !== a + (-b)`
impl Additive for Unorm8 {
    fn plus(self, rhs: Self) -> Self {
        Unorm8(self.0.saturating_add(rhs.0))
    }
    fn minus(self, rhs: Self) -> Self {
        Unorm8(self.0.saturating_sub(rhs.0))
    }
    fn zero() -> Self {
        Self::ZERO
    }
    fn negate(self) -> Self {
        Unorm8(0u8.saturating_sub(self.0))
    }
}

/// Fast rounding multiplication
///
/// Unlawful in the following ways:
/// - `(a*b)*c !== a*(b*c)` due to rounding error
/// - `a*(b+c) !== a*b + a*c` due to rounding and saturation
/// - `from_integer` not homomorphic because of saturation
impl Ring for Unorm8 {
    fn mult(self, rhs: Self) -> Self {
        self.mult_blinn(rhs)
    }
    fn one() -> Self {
        Self::ONE
    }
    fn from_integer(i: isize) -> Self {
        if i <= 0 {
            Self::ZERO
        } else {
            Self::ONE
        }
    }
}

impl_additive_ops!([] Unorm8);
impl_ring_ops!([] Unorm8);

#[cfg(any(test, feature = "proptest"))]
mod proptest_impls {
    use super::Unorm8;
    use proptest::prelude::*;

    impl Arbitrary for Unorm8 {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_: ()) -> Self::Strategy {
            any::<u8>().prop_map(Unorm8).boxed()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn mult_blinn_matches_ref(a: Unorm8, b: Unorm8) {
            prop_assert_eq!(a.mult_blinn(b), a.mult_ref(b));
        }

        #[test]
        fn mult_identity(a: Unorm8) {
            prop_assert_eq!(a * Unorm8::ONE, a);
        }

        #[test]
        fn mult_zero(a: Unorm8) {
            prop_assert_eq!(a * Unorm8::ZERO, Unorm8::ZERO);
        }

        #[test]
        fn mult_commutative(a: Unorm8, b: Unorm8) {
            prop_assert_eq!(a * b, b * a);
        }

        #[test]
        fn plus_commutative(a: Unorm8, b: Unorm8) {
            prop_assert_eq!(a + b, b + a);
        }

        #[test]
        fn plus_associative(a: Unorm8, b: Unorm8, c: Unorm8) {
            prop_assert_eq!((a + b) + c, a + (b + c));
        }

        #[test]
        fn plus_identity(a: Unorm8) {
            prop_assert_eq!(a + Unorm8::ZERO, a);
        }

        #[test]
        fn minus_identity(a: Unorm8) {
            prop_assert_eq!(a - a, Unorm8::ZERO);
        }
    }

    #[test]
    fn from_integer_endpoints() {
        assert_eq!(Unorm8::from_integer(0), Unorm8::ZERO);
        assert_eq!(Unorm8::from_integer(1), Unorm8::ONE);
    }

    #[test]
    fn encode_lut_matches_powf() {
        for i in 0..256 {
            let expected = ((i as f64 / 255.0).powf(1.0 / GAMMA) * 255.0).round() as u8;
            assert_eq!(ENCODE_LUT[i], expected, "mismatch at i={i}");
        }
    }

    #[test]
    fn decode_lut_matches_powf() {
        for i in 0..256 {
            let expected = ((i as f64 / 255.0).powf(GAMMA) * 255.0).round() as u8;
            assert_eq!(DECODE_LUT[i], expected, "mismatch at i={i}");
        }
    }
}
