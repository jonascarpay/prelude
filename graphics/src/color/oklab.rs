use prelude::{
    algebra::{
        abstract_::{Additive, VectorSpace},
        Ring,
    },
    impl_additive_ops, impl_vector_space_mul,
};

use crate::color::{
    lin_rgb::LinRgb,
    oklch::{Oklch, Oklch64},
};

#[derive(Clone, Copy, Debug)]
pub struct Oklab<T> {
    pub l: T,
    pub a: T,
    pub b: T,
}

pub type Oklab64 = Oklab<f64>;

impl Oklab64 {
    pub const fn to_rgb64(self) -> LinRgb<f64> {
        oklab64_to_rgb64(self)
    }

    pub fn to_oklch64(self) -> Oklch64 {
        let Oklab { l, a, b } = self;
        Oklch {
            l,
            c: (a * a + b * b).sqrt(),
            h: b.atan2(a),
        }
    }
}

pub const fn oklab64_to_rgb64(Oklab { l, a, b }: Oklab64) -> LinRgb<f64> {
    let l_ = 0.2158037573f64.mul_add(b, 0.3963377774f64.mul_add(a, l));
    let m_ = (-0.0638541728f64).mul_add(b, (-0.1055613458f64).mul_add(a, l));
    let s_ = (-1.2914855480f64).mul_add(b, (-0.0894841775f64).mul_add(a, l));
    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;
    LinRgb {
        r: 0.2309699292f64.mul_add(s, 4.0767416621f64.mul_add(l, -3.3077115913 * m)),
        g: (-0.3413193965f64).mul_add(s, (-1.2684380046f64).mul_add(l, 2.6097574011 * m)),
        b: 1.7076147010f64.mul_add(s, (-0.0041960863f64).mul_add(l, -0.7034186147 * m)),
    }
}

pub fn rgb64_to_oklab64(LinRgb { r, g, b }: LinRgb<f64>) -> Oklab64 {
    let l = 0.0514459929f64.mul_add(b, 0.4122214708f64.mul_add(r, 0.5363325363 * g));
    let m = 0.1073969566f64.mul_add(b, 0.2119034982f64.mul_add(r, 0.6806995451 * g));
    let s = 0.6299787005f64.mul_add(b, 0.0883024619f64.mul_add(r, 0.2817188376 * g));
    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();
    Oklab {
        l: (-0.0040720468f64).mul_add(s_, 0.2104542553f64.mul_add(l_, 0.7936177850 * m_)),
        a: 0.4505937099f64.mul_add(s_, 1.9779984951f64.mul_add(l_, -2.4285922050 * m_)),
        b: (-0.8086757660f64).mul_add(s_, 0.0259040371f64.mul_add(l_, 0.7827717662 * m_)),
    }
}

impl<T: Additive> Additive for Oklab<T> {
    fn plus(self, rhs: Self) -> Self {
        Oklab {
            l: self.l.plus(rhs.l),
            a: self.a.plus(rhs.a),
            b: self.b.plus(rhs.b),
        }
    }

    fn zero() -> Self {
        Oklab {
            l: T::zero(),
            a: T::zero(),
            b: T::zero(),
        }
    }

    fn negate(self) -> Self {
        Oklab {
            l: self.l.negate(),
            a: self.a.negate(),
            b: self.b.negate(),
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Oklab {
            l: self.l.minus(rhs.l),
            a: self.a.minus(rhs.a),
            b: self.b.minus(rhs.b),
        }
    }

    fn incr_by(&mut self, addend: Self) {
        self.l.incr_by(addend.l);
        self.a.incr_by(addend.a);
        self.b.incr_by(addend.b);
    }
}

impl<T: Ring> VectorSpace for Oklab<T> {
    type Scalar = T;

    fn scale(self, c: Self::Scalar) -> Self {
        Oklab {
            l: self.l.mult(c.clone()),
            a: self.a.mult(c.clone()),
            b: self.b.mult(c),
        }
    }
}

impl_additive_ops!([T: Additive] Oklab<T>);
impl_vector_space_mul!([T: Ring] Oklab<T>);
