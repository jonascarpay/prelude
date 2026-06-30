use crate::color::oklab::{Oklab, Oklab64};

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
}
