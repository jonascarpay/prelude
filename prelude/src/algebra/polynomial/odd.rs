use crate::{
    algebra::{
        abstract_::Functor, polynomial::even::EvenPolynomial, zero, Additive, Ring, VectorSpace,
    },
    impl_additive_ops, impl_vector_space_ops,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OddPolynomial<T, const COEFFS: usize> {
    pub coefficients: [T; COEFFS],
}

pub type OddCubic<T> = OddPolynomial<T, 2>;
pub type OddQuintic<T> = OddPolynomial<T, 3>;

impl<T: Additive, const COEFFS: usize> OddPolynomial<T, COEFFS> {
    pub fn from_coefficients(coefficients: [T; COEFFS]) -> Self {
        OddPolynomial { coefficients }
    }

    pub fn evaluate_ring(&self, x: T) -> T
    where
        T: Ring,
    {
        eval_odd_horner(&self.coefficients, x)
    }

    pub fn derivative_ring(self) -> EvenPolynomial<T, COEFFS>
    where
        T: Ring,
    {
        EvenPolynomial {
            coefficients: core::array::from_fn(|i| {
                self.coefficients[i].clone().imult((2 * i + 1) as isize)
            }),
        }
    }

    pub fn derivative_vector_space(self) -> EvenPolynomial<T, COEFFS>
    where
        T: VectorSpace,
    {
        EvenPolynomial {
            coefficients: core::array::from_fn(|i| {
                self.coefficients[i].clone().iscale((2 * i + 1) as isize)
            }),
        }
    }
}

#[inline(always)]
pub fn eval_odd_horner<T: Ring>(coeffs: &[T], x: T) -> T {
    let x2 = x.clone().mult(x.clone());
    coeffs
        .iter()
        .rev()
        .fold(T::zero(), |acc, a| acc.mult(x2.clone()).plus(a.clone()))
        .mult(x)
}

impl<T: Additive, const COEFFS: usize> Additive for OddPolynomial<T, COEFFS> {
    fn plus(self, rhs: Self) -> Self {
        OddPolynomial {
            coefficients: self.coefficients.plus(rhs.coefficients),
        }
    }

    fn zero() -> Self {
        OddPolynomial {
            coefficients: zero(),
        }
    }

    fn negate(self) -> Self {
        OddPolynomial {
            coefficients: self.coefficients.negate(),
        }
    }
}

impl<T: Ring, const COEFFS: usize> VectorSpace for OddPolynomial<T, COEFFS> {
    type Scalar = T;

    fn scale(self, c: Self::Scalar) -> Self {
        OddPolynomial {
            coefficients: self.coefficients.scale(c),
        }
    }
}

impl<T, const COEFFS: usize> Functor for OddPolynomial<T, COEFFS> {
    type Param = T;

    type Output<B> = OddPolynomial<B, COEFFS>;

    fn map<B, F: FnMut(Self::Param) -> B>(self, f: F) -> OddPolynomial<B, COEFFS> {
        OddPolynomial {
            coefficients: self.coefficients.map(f),
        }
    }
}
impl_additive_ops!([T: Additive, const COEFFS: usize] OddPolynomial<T, COEFFS>);
impl_vector_space_ops!([T: Ring + Copy, const COEFFS: usize] OddPolynomial<T, COEFFS>);
