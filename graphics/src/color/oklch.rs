use crate::color::{
    lin_rgb::LinRgb,
    oklab::{Oklab, Oklab64},
    srgb::Srgb,
};

#[derive(Clone, Copy, Debug)]
pub struct Oklch<T> {
    pub l: T,
    pub c: T,
    pub h: T,
}

pub type Oklch64 = Oklch<f64>;

impl Oklch64 {
    pub fn to_oklab64(self) -> Oklab64 {
        Oklab {
            l: self.l,
            a: self.c * self.h.cos(),
            b: self.c * self.h.sin(),
        }
    }
    pub fn to_rgb64(self) -> LinRgb<f64> {
        self.to_oklab64().to_rgb64()
    }
    pub fn to_srgb(self) -> Srgb {
        self.to_rgb64().to_srgb()
    }
}
