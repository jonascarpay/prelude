use crate::{
    algebra::{
        abstract_::Functor,
        polynomial::odd::{OddCubic, OddPolynomial},
        zero, Additive, Ring, VectorSpace,
    },
    impl_additive_ops, impl_vector_space_ops,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EvenPolynomial<T, const COEFFS: usize> {
    pub coefficients: [T; COEFFS],
}

pub type EvenQuadratic<T> = EvenPolynomial<T, 2>;
pub type EvenQuartic<T> = EvenPolynomial<T, 3>;

impl<T: Additive, const COEFFS: usize> EvenPolynomial<T, COEFFS> {
    pub fn from_scalar(x: T) -> Self {
        const {
            assert!(COEFFS > 0, "Empty polynomial");
        }
        let mut res: Self = zero();
        res.coefficients[0] = x;
        res
    }

    pub fn from_coefficients(coefficients: [T; COEFFS]) -> Self {
        EvenPolynomial { coefficients }
    }

    pub fn evaluate_ring(&self, x: T) -> T
    where
        T: Ring,
    {
        eval_even_horner(&self.coefficients, x)
    }

    fn generic_derivative_ring<const OUT: usize>(self) -> OddPolynomial<T, OUT>
    where
        T: Ring,
    {
        OddPolynomial {
            coefficients: core::array::from_fn(|i| {
                self.coefficients[i + 1].clone().imult(2 * (i + 1) as isize)
            }),
        }
    }

    fn generic_derivative_vector_space<const OUT: usize>(self) -> OddPolynomial<T, OUT>
    where
        T: VectorSpace,
    {
        OddPolynomial {
            coefficients: core::array::from_fn(|i| {
                self.coefficients[i + 1]
                    .clone()
                    .iscale((2 * (i + 1)) as isize)
            }),
        }
    }
}

#[inline(always)]
pub fn eval_even_horner<T: Ring>(coeffs: &[T], x: T) -> T {
    let x2 = x.clone().mult(x.clone());
    coeffs
        .iter()
        .rev()
        .fold(T::zero(), |acc, a| acc.mult(x2.clone()).plus(a.clone()))
}

impl<T: Additive, const COEFFS: usize> Additive for EvenPolynomial<T, COEFFS> {
    const ZERO: Self = EvenPolynomial {
        coefficients: zero(),
    };

    fn plus(self, rhs: Self) -> Self {
        EvenPolynomial {
            coefficients: self.coefficients.plus(rhs.coefficients),
        }
    }

    fn negate(self) -> Self {
        EvenPolynomial {
            coefficients: self.coefficients.negate(),
        }
    }
}

impl<T: Ring, const COEFFS: usize> VectorSpace for EvenPolynomial<T, COEFFS> {
    type Scalar = T;

    fn scale(self, c: Self::Scalar) -> Self {
        EvenPolynomial {
            coefficients: self.coefficients.scale(c),
        }
    }
}

impl<T, const COEFFS: usize> Functor for EvenPolynomial<T, COEFFS> {
    type Param = T;

    type Output<B> = EvenPolynomial<B, COEFFS>;

    fn map<B, F: FnMut(Self::Param) -> B>(self, f: F) -> EvenPolynomial<B, COEFFS> {
        EvenPolynomial {
            coefficients: self.coefficients.map(f),
        }
    }
}

impl<T> EvenQuartic<T> {
    // TODO to_dense
    pub fn derivative_ring(self) -> OddCubic<T>
    where
        T: Ring,
    {
        self.generic_derivative_ring()
    }
    pub fn derivative_vector_space(self) -> OddCubic<T>
    where
        T: VectorSpace,
    {
        self.generic_derivative_vector_space()
    }
}
impl_additive_ops!([T: Additive, const COEFFS: usize] EvenPolynomial<T, COEFFS>);
impl_vector_space_ops!([T: Ring + Copy, const COEFFS: usize] EvenPolynomial<T, COEFFS>);
