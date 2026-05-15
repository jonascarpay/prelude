use std::ops::{RangeBounds, RangeTo};

use crate::random::rng::Rng;

pub trait Uniform<T> {
    fn draw<R: Rng>(self, rng: &mut R) -> T;
}

pub trait UniformExt<T, UR, UT> {
    fn uniform(&mut self, range: UR) -> UT;
}

impl<T: Rng, UR: Uniform<UT>, UT> UniformExt<T, UR, UT> for T {
    fn uniform(&mut self, range: UR) -> UT {
        range.draw(self)
    }
}

impl Uniform<usize> for RangeTo<usize> {
    fn draw<R: Rng>(self, rng: &mut R) -> usize {
        loop {
            let r = rng.next_u64() as usize;
            if r < self.end {
                return r;
            }
        }
    }
}
