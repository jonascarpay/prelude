use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Ring, VectorSpace,
};

pub struct Complex<T> {
    pub s: T,
    pub xy: T,
}

impl<T: Ring + Copy> Complex<T> {
    pub fn i() -> Self {
        Self::xy()
    }
    pub fn xy() -> Self {
        Complex {
            s: T::zero(),
            xy: T::one(),
        }
    }
    pub fn basis() -> [Self; 2] {
        [Complex::one(), Self::xy()]
    }
}

impl<T: Additive> Additive for Complex<T> {
    fn minus(self, rhs: Self) -> Self {
        self.plus(rhs.negate())
    }

    fn plus(self, rhs: Self) -> Self {
        Complex {
            s: self.s.plus(rhs.s),
            xy: self.xy.plus(rhs.xy),
        }
    }

    fn zero() -> Self {
        Complex {
            s: T::zero(),
            xy: T::zero(),
        }
    }

    fn negate(self) -> Self {
        Complex {
            s: self.s.negate(),
            xy: self.xy.negate(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.xy.is_zero()
    }
}

impl<T: Ring + Copy> VectorSpace for Complex<T> {
    type Over = T;

    fn scale(self, c: T) -> Self {
        Complex {
            s: self.s.mult(c),
            xy: self.xy.mult(c),
        }
    }
}

impl<T: Ring + Copy> Ring for Complex<T> {
    fn mult(self, rhs: Self) -> Self {
        // (a + bi) (c + di)
        // ac + adi + bci - bd
        // (ac - bd)1 + (ad + bc)i
        let Complex { s: a, xy: b } = self;
        let Complex { s: c, xy: d } = rhs;
        Complex {
            s: a.mult(c).minus(b.mult(d)),
            xy: a.mult(d).plus(b.mult(c)),
        }
    }

    fn one() -> Self {
        Complex {
            s: T::one(),
            xy: T::zero(),
        }
    }

    fn from_integer(i: isize) -> Self {
        Complex {
            s: T::from_integer(i),
            xy: T::zero(),
        }
    }
}

impl_additive_ops!([T: Additive] Complex<T>);
impl_vector_space_ops!([T: Ring + Copy] Complex<T>);
