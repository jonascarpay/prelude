use crate::color::unorm8::Unorm8;
use prelude::algebra::abstract_::{Additive, Ring, VectorSpace};
use prelude::{impl_additive_ops, impl_vector_space_mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinRgb {
    pub r: Unorm8,
    pub g: Unorm8,
    pub b: Unorm8,
}

impl LinRgb {
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

    pub const fn new(r: Unorm8, g: Unorm8, b: Unorm8) -> Self {
        Self { r, g, b }
    }
}

impl Additive for LinRgb {
    fn plus(self, rhs: Self) -> Self {
        Self {
            r: self.r.plus(rhs.r),
            g: self.g.plus(rhs.g),
            b: self.b.plus(rhs.b),
        }
    }
    fn minus(self, rhs: Self) -> Self {
        Self {
            r: self.r.minus(rhs.r),
            g: self.g.minus(rhs.g),
            b: self.b.minus(rhs.b),
        }
    }
    fn zero() -> Self {
        Self::BLACK
    }
    fn negate(self) -> Self {
        Self {
            r: self.r.negate(),
            g: self.g.negate(),
            b: self.b.negate(),
        }
    }
    fn is_zero(&self) -> bool {
        self.r.is_zero() && self.g.is_zero() && self.b.is_zero()
    }
}

impl VectorSpace for LinRgb {
    type Scalar = Unorm8;
    fn scale(self, c: Unorm8) -> Self {
        Self {
            r: self.r.mult(c),
            g: self.g.mult(c),
            b: self.b.mult(c),
        }
    }
}

impl_additive_ops!([] LinRgb);
impl_vector_space_mul!([] LinRgb);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    impl Arbitrary for LinRgb {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_: ()) -> Self::Strategy {
            (any::<Unorm8>(), any::<Unorm8>(), any::<Unorm8>())
                .prop_map(|(r, g, b)| LinRgb { r, g, b })
                .boxed()
        }
    }

    proptest! {
        #[test]
        fn plus_commutative(a: LinRgb, b: LinRgb) {
            prop_assert_eq!(a + b, b + a);
        }

        #[test]
        fn plus_associative(a: LinRgb, b: LinRgb, c: LinRgb) {
            prop_assert_eq!((a + b) + c, a + (b + c));
        }

        #[test]
        fn plus_identity(a: LinRgb) {
            prop_assert_eq!(a + LinRgb::BLACK, a);
        }

        #[test]
        fn scale_identity(a: LinRgb) {
            prop_assert_eq!(a * Unorm8::ONE, a);
        }

        #[test]
        fn scale_zero(a: LinRgb) {
            prop_assert_eq!(a * Unorm8::ZERO, LinRgb::BLACK);
        }

        #[test]
        fn scale_componentwise(a: LinRgb, c: Unorm8) {
            let scaled = a * c;
            prop_assert_eq!(scaled.r, a.r * c);
            prop_assert_eq!(scaled.g, a.g * c);
            prop_assert_eq!(scaled.b, a.b * c);
        }
    }
}
