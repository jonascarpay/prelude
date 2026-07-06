use core::ops::{Deref, DerefMut};

use crate::{
    algebra::{
        abstract_::{additive::zero, Additive, Functor, VectorSpace},
        Ring,
    },
    impl_additive_ops, impl_vector_space_ops,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DensePolynomial<T, const COEFFS: usize> {
    pub coefficients: [T; COEFFS],
}

pub type Linear<T> = DensePolynomial<T, 2>;
pub type Quadratic<T> = DensePolynomial<T, 3>;
pub type Cubic<T> = DensePolynomial<T, 4>;
pub type Quartic<T> = DensePolynomial<T, 5>;
pub type Quintic<T> = DensePolynomial<T, 6>;

impl<T: Additive, const COEFFS: usize> DensePolynomial<T, COEFFS> {
    pub fn from_scalar(x: T) -> Self {
        const {
            assert!(COEFFS > 0, "Empty polynomial");
        }
        let mut res: Self = zero();
        res.coefficients[0] = x;
        res
    }

    pub fn from_coefficients(coefficients: [T; COEFFS]) -> Self {
        DensePolynomial { coefficients }
    }

    pub fn from_integer_coefficients(coefficients: [isize; COEFFS]) -> Self
    where
        T: Ring,
    {
        DensePolynomial {
            coefficients: coefficients.map(|i| T::from_integer(i)),
        }
    }

    fn mult_x(self) -> Self {
        DensePolynomial {
            coefficients: std::array::from_fn(|i| {
                if i == 0 {
                    T::zero()
                } else {
                    self.coefficients[i - 1].clone()
                }
            }),
        }
    }

    pub fn from_roots<const NROOTS: usize>(roots: [T; NROOTS]) -> Self
    where
        T: Ring,
    {
        Self::from_scale_roots(T::one(), roots)
    }

    pub fn from_scale_roots<const NROOTS: usize>(scale: T, roots: [T; NROOTS]) -> Self
    where
        T: Ring,
    {
        const {
            assert!(COEFFS > NROOTS, "Degree too low for number of roots");
        }

        let mut res: Self = Self::from_scalar(scale);

        for root in roots {
            res = res.clone().mult_x().plus(res.scale(root.negate()));
        }

        res
    }

    pub fn evaluate_ring(&self, x: T) -> T
    where
        T: Ring,
    {
        eval_horner(&self.coefficients, x)
    }

    pub fn evaluate_vector_space(&self, x: T::Scalar) -> T
    where
        T: VectorSpace,
    {
        eval_vector_space(&self.coefficients, x)
    }

    pub fn evaluate_vector_space_horner(&self, x: T::Scalar) -> T
    where
        T: VectorSpace,
    {
        eval_vector_space_horner(&self.coefficients, x)
    }

    fn generic_derivative_ring<const OUT: usize>(self) -> DensePolynomial<T, OUT>
    where
        T: Ring,
    {
        DensePolynomial {
            coefficients: core::array::from_fn(|i| {
                self.coefficients[i + 1].clone().imult((i + 1) as isize)
            }),
        }
    }
    fn generic_derivative_vector_space<const OUT: usize>(self) -> DensePolynomial<T, OUT>
    where
        T: VectorSpace,
    {
        DensePolynomial {
            coefficients: core::array::from_fn(|i| {
                self.coefficients[i + 1].clone().iscale((i + 1) as isize)
            }),
        }
    }
}

impl<T: Additive, const COEFFS: usize> Additive for DensePolynomial<T, COEFFS> {
    fn plus(self, rhs: Self) -> Self {
        DensePolynomial {
            coefficients: self.coefficients.plus(rhs.coefficients),
        }
    }

    fn zero() -> Self {
        DensePolynomial {
            coefficients: zero(),
        }
    }

    fn negate(self) -> Self {
        DensePolynomial {
            coefficients: self.coefficients.negate(),
        }
    }
}

impl<T: Ring, const COEFFS: usize> VectorSpace for DensePolynomial<T, COEFFS> {
    type Scalar = T;

    fn scale(self, c: Self::Scalar) -> Self {
        DensePolynomial {
            coefficients: self.coefficients.scale(c),
        }
    }
}

impl<T, const COEFFS: usize> Deref for DensePolynomial<T, COEFFS> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.coefficients
    }
}

impl<T, const COEFFS: usize> DerefMut for DensePolynomial<T, COEFFS> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.coefficients
    }
}

impl<T, const COEFFS: usize> Functor for DensePolynomial<T, COEFFS> {
    type Param = T;

    type Output<B> = DensePolynomial<B, COEFFS>;

    fn map<B, F: FnMut(Self::Param) -> B>(self, f: F) -> DensePolynomial<B, COEFFS> {
        DensePolynomial {
            coefficients: self.coefficients.map(f),
        }
    }
}

impl<T> Quintic<T> {
    pub fn derivative_ring(self) -> Quartic<T>
    where
        T: Ring,
    {
        self.generic_derivative_ring()
    }
    pub fn derivative_vector_space(self) -> Quartic<T>
    where
        T: VectorSpace,
    {
        self.generic_derivative_vector_space()
    }
}

impl<T> Quartic<T> {
    pub fn derivative_ring(self) -> Cubic<T>
    where
        T: Ring,
    {
        self.generic_derivative_ring()
    }
    pub fn derivative_vector_space(self) -> Cubic<T>
    where
        T: VectorSpace,
    {
        self.generic_derivative_vector_space()
    }
}
impl<T> Cubic<T> {
    pub fn derivative_ring(self) -> Quadratic<T>
    where
        T: Ring,
    {
        self.generic_derivative_ring()
    }
    pub fn derivative_vector_space(self) -> Quadratic<T>
    where
        T: VectorSpace,
    {
        self.generic_derivative_vector_space()
    }
}

impl_additive_ops!([T: Additive, const COEFFS: usize] DensePolynomial<T, COEFFS>);
impl_vector_space_ops!([T: Ring + Copy, const COEFFS: usize] DensePolynomial<T, COEFFS>);

#[inline(always)]
pub fn eval_horner<T: Ring>(coeffs: &[T], x: T) -> T {
    coeffs
        .iter()
        .rev()
        .fold(T::zero(), |acc, a| acc.mult(x.clone()).plus(a.clone()))
}

#[inline(always)]
pub fn eval_vector_space_horner<V: VectorSpace>(coeffs: &[V], x: V::Scalar) -> V {
    coeffs
        .iter()
        .rev()
        .fold(V::zero(), |acc, c| acc.scale(x.clone()).plus(c.clone()))
}

#[inline(always)]
pub fn eval_vector_space<V: VectorSpace>(coeffs: &[V], x: V::Scalar) -> V {
    let mut acc = V::zero();
    let mut power = <V::Scalar as Ring>::one();
    for c in coeffs {
        acc = acc.plus(c.scaled(power.clone()));
        power = power.mult(x.clone());
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn from_roots_vanishes_at_roots(
            scale in -1000i64..=1000,
            roots in prop::array::uniform3(-1000i64..=1000),
        ) {
            let p = DensePolynomial::<i64, 4>::from_scale_roots(scale, roots);
            for r in roots {
                prop_assert_eq!(p.evaluate_ring(r), 0);
            }
        }
    }
}
