use crate::algebra::{abstract_::InnerProductSpace, geometric::complex::Complex};

use super::super::abstract_::{impl_additive_ops, impl_vector_space_ops, Additive, Ring, VectorSpace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// TODO Rename to Vec2
pub struct V2<T> {
    pub x: T,
    pub y: T,
}

pub const fn v2<T>(x: T, y: T) -> V2<T> {
    V2 { x, y }
}

impl<T> V2<T> {
    pub const fn new(x: T, y: T) -> V2<T> {
        V2 { x, y }
    }
    pub fn xunit() -> Self
    where
        T: Ring,
    {
        V2 {
            x: T::one(),
            y: T::zero(),
        }
    }
    pub fn yunit() -> Self
    where
        T: Ring,
    {
        V2 {
            x: T::zero(),
            y: T::one(),
        }
    }
    pub fn basis() -> [Self; 2]
    where
        T: Ring,
    {
        [Self::xunit(), Self::yunit()]
    }
    // TODO functor??
    pub fn map<Out, F: FnMut(T) -> Out>(self, mut f: F) -> V2<Out> {
        V2 {
            x: f(self.x),
            y: f(self.y),
        }
    }
    pub fn map_into<Out: From<T>>(self) -> V2<Out> {
        V2 {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
    pub fn pack(self) -> [T; 2] {
        [self.x, self.y]
    }
    pub fn unpack([x, y]: [T; 2]) -> Self {
        V2 { x, y }
    }
}

impl<T: Additive> Additive for V2<T> {
    fn plus(self, rhs: Self) -> Self {
        V2 {
            x: self.x.plus(rhs.x),
            y: self.y.plus(rhs.y),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        V2 {
            x: self.x.minus(rhs.x),
            y: self.y.minus(rhs.y),
        }
    }

    fn zero() -> Self {
        V2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn negate(self) -> Self {
        V2 {
            x: self.x.negate(),
            y: self.y.negate(),
        }
    }
}

impl<T: Ring + Clone> VectorSpace for V2<T> {
    type Scalar = T;

    fn scale(self, c: Self::Scalar) -> Self {
        V2 {
            x: self.x.mult(c.clone()),
            y: self.y.mult(c),
        }
    }
}

impl<T: Ring + Clone> InnerProductSpace for V2<T> {
    fn quadrance(self) -> Self::Scalar {
        self.x.squared().plus(self.y.squared())
    }

    fn inner(self, rhs: Self) -> Self::Scalar {
        self.x.mult(rhs.x).plus(self.y.mult(rhs.y))
    }
}

impl_additive_ops!([T: Additive] V2<T>);
impl_vector_space_ops!([T: Ring + Clone] V2<T>);

impl<T: Ring + Copy> std::ops::Mul<Complex<T>> for V2<T> {
    type Output = V2<T>;

    fn mul(self, rhs: Complex<T>) -> Self::Output {
        // (ax + by)(c + dxy)
        // axc + axdxy + byc + bydxy
        // acx + ady + bcy - bdx
        // (ac - bd)x + (ad + bc)y
        // why is this the exact same as complex mult???
        let V2 { x: a, y: b } = self;
        let Complex { s: c, xy: d } = rhs;
        V2 {
            x: a.mult(c).minus(b.mult(d)),
            y: a.mult(d).plus(b.mult(c)),
        }
    }
}

impl<T: Ring + Copy> std::ops::Mul<V2<T>> for Complex<T> {
    type Output = V2<T>;

    fn mul(self, rhs: V2<T>) -> Self::Output {
        // (c + dxy)(ax + by)
        // cax + cby + dxyax + dxyby
        // acx + bcy - ady + bdx
        // (ac + bd)x + (bc - ad)y
        let Complex { s: c, xy: d } = self;
        let V2 { x: a, y: b } = rhs;
        V2 {
            x: a.mult(c).plus(b.mult(d)),
            y: a.mult(d).minus(b.mult(c)),
        }
    }
}

impl<T: Ring + Copy> std::ops::Mul<V2<T>> for V2<T> {
    type Output = Complex<T>;

    fn mul(self, rhs: V2<T>) -> Self::Output {
        // (ax + by)(cx + dy)
        // axcx + axdy + bycx + bydy
        // ac + adxy - bcxy + bd
        // (ac + bd) + (ad - bc)xy
        let V2 { x: a, y: b } = rhs;
        let V2 { x: c, y: d } = self;
        Complex {
            s: a.mult(c).plus(b.mult(d)),
            xy: a.mult(d).minus(b.mult(c)),
        }
    }
}
