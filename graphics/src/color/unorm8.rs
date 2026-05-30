#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
            prop_assert_eq!(a.mult_blinn(Unorm8::ONE), a);
        }

        #[test]
        fn mult_zero(a: Unorm8) {
            prop_assert_eq!(a.mult_blinn(Unorm8::ZERO), Unorm8::ZERO);
        }

    }
}
