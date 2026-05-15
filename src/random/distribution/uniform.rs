use std::ops::{Range, RangeInclusive, RangeTo};

use crate::random::{distribution::Distribution, rng::Rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uniform<T>(pub T);

fn sample_lt_u64<R: Rng>(rng: &mut R, end: u64) -> u64 {
    assert!(end > 0);
    let thresh = u64::MAX - u64::MAX % end;
    loop {
        let r = rng.next_u64();
        if r < thresh {
            return r % end;
        }
    }
}

impl Distribution for Uniform<RangeTo<u64>> {
    type Output = u64;
    fn sample<R: Rng>(&self, rng: &mut R) -> Self::Output {
        sample_lt_u64(rng, self.0.end)
    }
}

impl Distribution for Uniform<Range<u64>> {
    type Output = u64;
    fn sample<R: Rng>(&self, rng: &mut R) -> Self::Output {
        let Range { start, end } = self.0;
        assert!(end > start);
        start + sample_lt_u64(rng, end - start)
    }
}

impl Distribution for Uniform<RangeInclusive<u64>> {
    type Output = u64;
    fn sample<R: Rng>(&self, rng: &mut R) -> Self::Output {
        let (start, end) = (*self.0.start(), *self.0.end());
        assert!(end >= start);
        match (end - start).checked_add(1) {
            Some(len) => start + sample_lt_u64(rng, len),
            None => rng.next_u64(),
        }
    }
}
