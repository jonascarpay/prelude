use prelude::algebra::abstract_::{Additive, Ring};
use prelude::{impl_additive_ops, impl_ring_ops};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// TODO make generic over T using FixedBase
// TODO move to prelude, generally useful not just for color
pub struct Unorm8(pub u8);

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
}

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
    fn is_zero(&self) -> bool {
        self.0 == 0
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for Unorm8 {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_: ()) -> Self::Strategy {
            any::<u8>().prop_map(Unorm8).boxed()
        }
    }

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
}
