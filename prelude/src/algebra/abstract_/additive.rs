use crate::algebra::abstract_::group::{Group, Monoid, Semigroup};

/// An additive group
pub trait Additive: Sized + Clone {
    // TODO: investigate whether it makes sense to have
    //   plus_ref: &T -> &T -> T
    //   plus_mut: &mut T -> T -> ()
    //   plus_mut_ref: &mut T -> &T -> ()
    // with defaults that just clone/core::mem::replace

    // TODO: investigate whether we want checked versions, instead of forcing wrapping behavior.

    /// An associative, commutative operation
    fn plus(self, rhs: Self) -> Self;

    /// The identity element for `plus`
    fn zero() -> Self;

    /// The inverse element for `plus`
    fn negate(self) -> Self;

    fn minus(self, rhs: Self) -> Self {
        self.plus(rhs.negate())
    }

    fn negated(&self) -> Self {
        self.clone().negate()
    }

    fn incr_by(&mut self, addend: Self) {
        *self = self.clone().plus(addend);
    }

    // Should this be a subtrait of `Group`?
    // Pro:
    //   - Nicely extends `Group` by only adding the commutativity
    //   - For numbers, it's the most natural group anyway
    //   - Combines iter sum/concat
    //   - Removes need for `AsAdditiveGroup`
    // Con:
    //  - `Linear` is both an additive group and a group w.r.t. function composition
    //  - `LinRgba` is both an additive group and a monoid w.r.t. `over`
    //  - Matrices/complex numbers are more interesting as a multiplicative group
    //  - Multiplicative group ambiguity
}

pub fn zero<T: Additive>() -> T {
    T::zero()
}

pub trait EqAdditive: Eq + Additive {
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn is_nonzero(&self) -> bool {
        !self.is_zero()
    }
}

pub trait OrderedAdditive: Ord + Additive {
    fn cmp_zero(&self) -> std::cmp::Ordering {
        self.cmp(&Self::zero())
    }

    fn is_positive(&self) -> bool {
        self.cmp_zero().is_gt()
    }

    fn is_negative(&self) -> bool {
        self.cmp_zero().is_lt()
    }

    fn is_nonnegative(&self) -> bool {
        self.cmp_zero().is_ge()
    }

    fn abs(self) -> Self {
        if self.is_negative() {
            self.negate()
        } else {
            self
        }
    }
}

impl<T: Eq + Additive> EqAdditive for T {}
impl<T: Ord + Additive> OrderedAdditive for T {}

/// newtype wrapper that turns an `Additive` into a `Group`
pub struct AsAdditiveGroup<T>(pub T);
// TODO Additive impl

impl<T: Additive> Semigroup for AsAdditiveGroup<T> {
    fn compose(self, rhs: Self) -> Self {
        AsAdditiveGroup(self.0.plus(rhs.0))
    }
}

impl<T: Additive> Monoid for AsAdditiveGroup<T> {
    fn identity() -> Self {
        AsAdditiveGroup(T::zero())
    }
}

impl<T: Additive> Group for AsAdditiveGroup<T> {
    fn inverse(self) -> Self {
        AsAdditiveGroup(self.0.negate())
    }
}

#[inline]
pub fn iter_sum<T: Additive, I: Iterator<Item = T>>(iter: I) -> T {
    iter.fold(T::zero(), T::plus)
}

/// Sum of an iterator, does not add 0 and the first element.
/// Tends to optimizer better for small fixed-size iterators, worse for large/dynamic ones.
#[inline]
pub fn iter_sum_reduce<T: Additive, I: Iterator<Item = T>>(iter: I) -> T {
    iter.reduce(T::plus).unwrap_or(T::zero())
}

pub fn step_by<T>(start: T, delta: T) -> ArithmeticSequence<T> {
    ArithmeticSequence::new(start, delta)
}

pub fn step_by_until<T: Additive + PartialOrd + Copy>(
    start: T,
    delta: T,
    end: T,
) -> impl Iterator<Item = T> {
    ArithmeticSequence::new(start, delta).take_while(move |v| *v < end)
}

pub struct ArithmeticSequence<T> {
    pub next: T,
    pub delta: T,
}

impl<T> ArithmeticSequence<T> {
    pub fn new(start: T, delta: T) -> Self {
        ArithmeticSequence { next: start, delta }
    }
}

impl<T: Additive> Iterator for ArithmeticSequence<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let next = self.next.clone();
        self.next.incr_by(self.delta.clone());
        Some(next)
    }
}

// impls //

impl Additive for () {
    fn plus(self, _rhs: Self) -> Self {}
    fn zero() -> Self {}
    fn negate(self) -> Self {}
}

impl<A, B> Additive for (A, B)
where
    A: Additive,
    B: Additive,
{
    fn plus(self, rhs: Self) -> Self {
        let (a0, a1) = self;
        let (b0, b1) = rhs;
        (a0.plus(b0), a1.plus(b1))
    }
    fn minus(self, rhs: Self) -> Self {
        let (a0, a1) = self;
        let (b0, b1) = rhs;
        (a0.minus(b0), a1.minus(b1))
    }

    fn zero() -> Self {
        (A::zero(), B::zero())
    }

    fn negate(self) -> Self {
        let (a0, a1) = self;
        (a0.negate(), a1.negate())
    }
}

impl<A, B, C> Additive for (A, B, C)
where
    A: Additive,
    B: Additive,
    C: Additive,
{
    fn plus(self, rhs: Self) -> Self {
        let (a0, a1, a2) = self;
        let (b0, b1, b2) = rhs;
        (a0.plus(b0), a1.plus(b1), a2.plus(b2))
    }
    fn minus(self, rhs: Self) -> Self {
        let (a0, a1, a2) = self;
        let (b0, b1, b2) = rhs;
        (a0.minus(b0), a1.minus(b1), a2.minus(b2))
    }

    fn zero() -> Self {
        (A::zero(), B::zero(), C::zero())
    }

    fn negate(self) -> Self {
        let (a0, a1, a2) = self;
        (a0.negate(), a1.negate(), a2.negate())
    }
}

impl<A, B, C, D> Additive for (A, B, C, D)
where
    A: Additive,
    B: Additive,
    C: Additive,
    D: Additive,
{
    fn plus(self, rhs: Self) -> Self {
        let (a0, a1, a2, a3) = self;
        let (b0, b1, b2, b3) = rhs;
        (a0.plus(b0), a1.plus(b1), a2.plus(b2), a3.plus(b3))
    }
    fn minus(self, rhs: Self) -> Self {
        let (a0, a1, a2, a3) = self;
        let (b0, b1, b2, b3) = rhs;
        (a0.minus(b0), a1.minus(b1), a2.minus(b2), a3.minus(b3))
    }

    fn zero() -> Self {
        (A::zero(), B::zero(), C::zero(), D::zero())
    }

    fn negate(self) -> Self {
        let (a0, a1, a2, a3) = self;
        (a0.negate(), a1.negate(), a2.negate(), a3.negate())
    }
}

impl<T: Additive, const N: usize> Additive for [T; N] {
    fn plus(self, rhs: Self) -> Self {
        // Confirmed this unrolls and vectorizes cleanly
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        std::array::from_fn(|_| lhs.next().unwrap().plus(rhs.next().unwrap()))
    }
    fn minus(self, rhs: Self) -> Self {
        // Confirmed this unrolls and vectorizes cleanly
        let mut lhs = self.into_iter();
        let mut rhs = rhs.into_iter();
        std::array::from_fn(|_| lhs.next().unwrap().minus(rhs.next().unwrap()))
    }

    fn zero() -> Self {
        std::array::from_fn(|_| T::zero())
    }

    fn negate(self) -> Self {
        self.map(T::negate)
    }
}

macro_rules! impl_additive_modular {
    ($t:ty) => {
        impl Additive for $t {
            fn plus(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }
            fn minus(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }
            fn zero() -> Self {
                0
            }
            fn negate(self) -> Self {
                self.wrapping_neg()
            }
        }
    };
}

macro_rules! impl_additive_float {
    ($t:ty) => {
        impl Additive for $t {
            fn plus(self, rhs: Self) -> Self {
                self + rhs
            }
            fn minus(self, rhs: Self) -> Self {
                self - rhs
            }
            fn zero() -> Self {
                0.0
            }
            fn negate(self) -> Self {
                -self
            }
        }
    };
}

/// Emit `Add`, `Sub`, `Neg` impls that forward to `Additive`.
///
/// Usage: `impl_additive_ops!([T: Additive] V2<T>);`
#[macro_export]
macro_rules! impl_additive_ops {
    ([$($g:tt)*] $t:ty) => {
        impl<$($g)*> ::core::ops::Add for $t {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                <Self as $crate::algebra::abstract_::Additive>::plus(self, rhs)
            }
        }
        impl<$($g)*> ::core::ops::Sub for $t {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                <Self as $crate::algebra::abstract_::Additive>::minus(self, rhs)
            }
        }
        impl<$($g)*> ::core::ops::Neg for $t {
            type Output = Self;
            fn neg(self) -> Self {
                <Self as $crate::algebra::abstract_::Additive>::negate(self)
            }
        }
    };
}

impl_additive_modular!(i8);
impl_additive_modular!(i16);
impl_additive_modular!(i32);
impl_additive_modular!(i64);
impl_additive_modular!(i128);
impl_additive_modular!(isize);
impl_additive_modular!(u8);
impl_additive_modular!(u16);
impl_additive_modular!(u32);
impl_additive_modular!(u64);
impl_additive_modular!(u128);
impl_additive_modular!(usize);
impl_additive_float!(f32);
impl_additive_float!(f64);
