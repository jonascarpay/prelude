use std::ops::Range;

use crate::algebra::{
    abstract_::{Functor, Group, InnerProductSpace, Monoid, Semigroup},
    geometric::{
        complex::Complex,
        vec3::{v3, V3},
    },
};

use super::super::abstract_::{
    impl_additive_ops, impl_vector_space_ops, Additive, Ring, VectorSpace,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// TODO Rename to Vec2
pub struct V2<T> {
    pub x: T,
    pub y: T,
}

pub const fn v2<T>(x: T, y: T) -> V2<T> {
    V2 { x, y }
}

impl<T: Ring> V2<T> {
    pub const ZERO: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
    };
    pub const X: Self = V2 {
        x: T::ONE,
        ..Self::ZERO
    };
    pub const Y: Self = V2 {
        y: T::ONE,
        ..Self::ZERO
    };
    pub const BASIS: [Self; 2] = [Self::X, Self::Y];

    pub const fn new(x: T, y: T) -> V2<T> {
        V2 { x, y }
    }
    pub fn pack(self) -> [T; 2] {
        [self.x, self.y]
    }
    pub fn unpack([x, y]: [T; 2]) -> Self {
        V2 { x, y }
    }
    pub fn pad(self, z: T) -> V3<T> {
        let V2 { x, y } = self;
        v3(x, y, z)
    }

    pub fn in_bounds(self, bounds: Range<V2<T>>) -> bool
    where
        T: PartialOrd,
    {
        (bounds.start.x <= self.x && self.x < bounds.end.x)
            && (bounds.start.y <= self.y && self.y < bounds.end.y)
    }

    pub fn into_complex(self) -> Complex<T> {
        Complex {
            s: self.x,
            xy: self.y,
        }
    }
}

impl<T> Functor for V2<T> {
    type Param = T;
    type Output<B> = V2<B>;
    fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> V2<B> {
        V2 {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl<T: Additive> Additive for V2<T> {
    const ZERO: Self = Self::ZERO;
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

impl<T: Semigroup> Semigroup for V2<T> {
    fn compose(self, rhs: Self) -> Self {
        v2(self.x.compose(rhs.x), self.y.compose(rhs.y))
    }
}

impl<T: Monoid> Monoid for V2<T> {
    fn identity() -> Self {
        v2(T::identity(), T::identity())
    }
}

impl<T: Group> Group for V2<T> {
    fn inverse(self) -> Self {
        v2(self.x.inverse(), self.y.inverse())
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
