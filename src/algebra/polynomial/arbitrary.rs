use super::super::abstract_::{
    Additive, Curve, DifferentiableCurve, Ring, VectorSpace, impl_additive_ops, impl_ring_ops,
    impl_vector_space_ops,
};

pub struct Polynomial<T> {
    // The last element should be nonzero
    // TODO: smallvec
    coeffs: Vec<T>, // self.coeffs.last.is_none_or(!= zero)
}

impl<T> Polynomial<T> {
    pub fn order(&self) -> usize {
        self.coeffs.len() - 1
    }
    pub fn get(&self, coeff: usize) -> T
    where
        T: Ring + Copy,
    {
        self.coeffs.get(coeff).map_or(T::zero(), |a| *a)
    }
    fn shrink(self) -> Self
    where
        T: Additive,
    {
        let mut coeffs = self.coeffs;
        let capacity = coeffs
            .iter()
            .enumerate()
            .rev()
            .find_map(|(ix, c)| if c.is_zero() { None } else { Some(ix + 1) })
            .unwrap_or(0);
        coeffs.shrink_to(capacity);
        Polynomial { coeffs }
    }
}

impl<T: Additive + Copy> Additive for Polynomial<T> {
    fn zero() -> Self {
        Polynomial { coeffs: Vec::new() }
    }
    fn plus(self, rhs: Self) -> Self {
        let (mut coeffs, addend) = if self.coeffs.len() >= rhs.coeffs.len() {
            (self.coeffs, rhs.coeffs)
        } else {
            (rhs.coeffs, self.coeffs)
        };
        for (i, x) in addend.into_iter().enumerate() {
            let r = &mut coeffs[i];
            *r = r.plus(x);
        }
        Polynomial { coeffs }.shrink()
    }
    fn minus(self, rhs: Self) -> Self {
        let (mut coeffs, addend) = if self.coeffs.len() >= rhs.coeffs.len() {
            (self.coeffs, rhs.coeffs)
        } else {
            (rhs.coeffs, self.coeffs)
        };
        for (i, x) in addend.into_iter().enumerate() {
            let r = &mut coeffs[i];
            *r = r.minus(x);
        }
        Polynomial { coeffs }.shrink()
    }
    fn negate(self) -> Self {
        Polynomial {
            coeffs: self.coeffs.into_iter().map(Additive::negate).collect(),
        }
    }
    fn is_zero(&self) -> bool {
        self.coeffs.len().is_zero()
    }
}

impl<T: Ring + Copy + PartialEq> VectorSpace for Polynomial<T> {
    type Over = T;
    fn scale(self, c: T) -> Self {
        if c.is_zero() {
            Self::zero()
        } else {
            Polynomial {
                coeffs: self.coeffs.into_iter().map(|a| a.mult(c)).collect(),
            }
        }
    }
}

impl<T: Ring + Copy> Ring for Polynomial<T> {
    fn mult(self, rhs: Self) -> Self {
        let n = self.order() + rhs.order() + 1;
        let mut coeffs = Vec::with_capacity(n);
        coeffs.resize_with(n, || T::zero());

        for (i, x) in self.coeffs.iter().enumerate() {
            for (j, y) in rhs.coeffs.iter().enumerate() {
                coeffs[i + j] = x.mult(*y);
            }
        }
        Polynomial { coeffs }
    }
    fn one() -> Self {
        Polynomial {
            coeffs: vec![T::one()],
        }
    }
    fn from_integer(i: isize) -> Self {
        Polynomial {
            coeffs: vec![T::from_integer(i)],
        }
    }
}

impl_additive_ops!([T: Additive + Copy] Polynomial<T>);
impl_vector_space_ops!([T: Ring + Copy + PartialEq] Polynomial<T>);
impl_ring_ops!([T: Ring + Copy] Polynomial<T>);

impl<T: Ring + Copy> Curve for Polynomial<T> {
    type Domain = T;
    type Range = T;
    fn evaluate(self, x: T) -> T {
        let mut res = T::zero();
        let mut term = T::one();
        for c in self.coeffs {
            res = res.plus(c.mult(term));
            term = term.mult(x);
        }
        res
    }
}

impl<T: Ring + Copy> DifferentiableCurve for Polynomial<T> {
    type Derivative = Self;
    fn derivative(self) -> Self::Derivative {
        Polynomial {
            coeffs: self
                .coeffs
                .into_iter()
                .enumerate()
                .skip(1)
                .map(|(i, x)| x.mult(T::from_integer(i as isize)))
                .collect(),
        }
    }
}
