use prelude::{
    algebra::{
        abstract_::{Additive, Ring, VectorSpace},
        numeric::unorm8::Unorm8,
    },
    impl_additive_ops, impl_vector_space_mul,
};

pub fn rgb<T>(r: T, g: T, b: T) -> LinRgb<T> {
    LinRgb::new(r, g, b)
}

/// Linear RGB, generic over the per-channel pixel type `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinRgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

/// 8-bit linear RGB — the storage/compositing representation.
pub type LinRgb8 = LinRgb<Unorm8>;

impl<T> LinRgb<T> {
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: Ring> LinRgb<T> {
    pub fn white() -> Self {
        rgb(T::one(), T::one(), T::one())
    }
    pub fn black() -> Self {
        rgb(T::zero(), T::zero(), T::zero())
    }
}

impl LinRgb8 {
    pub fn pack(self) -> [u8; 3] {
        [self.r.0, self.g.0, self.b.0]
    }
}

impl<T: Additive> Additive for LinRgb<T> {
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
        Self {
            r: T::zero(),
            g: T::zero(),
            b: T::zero(),
        }
    }
    fn negate(self) -> Self {
        Self {
            r: self.r.negate(),
            g: self.g.negate(),
            b: self.b.negate(),
        }
    }
}

impl<T: Ring> VectorSpace for LinRgb<T> {
    type Scalar = T;
    fn scale(self, c: T) -> Self {
        Self {
            r: self.r.mult(c.clone()),
            g: self.g.mult(c.clone()),
            b: self.b.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] LinRgb<T>);
impl_vector_space_mul!([T: Ring] LinRgb<T>);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    impl<T: Arbitrary + 'static> Arbitrary for LinRgb<T> {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(_: ()) -> Self::Strategy {
            (any::<T>(), any::<T>(), any::<T>())
                .prop_map(|(r, g, b)| LinRgb { r, g, b })
                .boxed()
        }
    }

    proptest! {
        // Explicitly testing LinRgb8 here
        #[test]
        fn plus_commutative(a: LinRgb8, b: LinRgb8) {
            prop_assert_eq!(a + b, b + a);
        }

        #[test]
        fn plus_associative(a: LinRgb8, b: LinRgb8, c: LinRgb8) {
            prop_assert_eq!((a + b) + c, a + (b + c));
        }

        #[test]
        fn plus_identity(a: LinRgb8) {
            prop_assert_eq!(a + LinRgb::black(), a);
        }

        #[test]
        fn scale_identity(a: LinRgb8) {
            prop_assert_eq!(a * Unorm8::ONE, a);
        }

        #[test]
        fn scale_zero(a: LinRgb8) {
            prop_assert_eq!(a * Unorm8::ZERO, LinRgb::zero());
        }

        #[test]
        fn scale_componentwise(a: LinRgb8, c: Unorm8) {
            let scaled = a * c;
            prop_assert_eq!(scaled.r, a.r * c);
            prop_assert_eq!(scaled.g, a.g * c);
            prop_assert_eq!(scaled.b, a.b * c);
        }
    }
}
