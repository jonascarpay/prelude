use std::ops::RangeBounds;

use crate::random::rng::Rng;

pub trait Uniform<T>: RangeBounds<T> {
    fn draw<R: Rng>(&mut self) -> T;
}
