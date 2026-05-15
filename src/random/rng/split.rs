use crate::random::rng::fast::FastRng;

#[derive(Clone, Debug)]
/// A splittable RNG, based on SplitMix64.
/// A hair slower and less random than FastRng, but supports splitting.
/// For the best of both worlds, use `SplitRng` as a seed to generate `FastRng`s using `SplitRng::next_rng`, which is also what `FastRng::new` does.
pub struct SplitRng {
    // https://xoshiro.di.unimi.it/splitmix64.c
    state: u64,
    gamma: u64,
}

// TODO decide if this should be an RNG in its own right, or only serve to bootstrap FastRng

impl SplitRng {
    pub const fn new(seed: u64) -> Self {
        SplitRng {
            state: seed,
            gamma: 0x9e3779b97f4a7c15,
        }
    }
    pub const fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(self.gamma);
        let mut z = self.state;
        z ^= z >> 30;
        z = z.wrapping_mul(0xbf58476d1ce4e5b9);
        z ^= z >> 27;
        z = z.wrapping_mul(0x94d049bb133111eb);
        z ^= z >> 31;
        z
    }

    pub const fn next_rng(&mut self) -> FastRng {
        FastRng {
            s0: self.next_u64(),
            s1: self.next_u64(),
            s2: self.next_u64(),
            s3: self.next_u64(),
        }
    }

    // TODO next_split_rng?
    // TODO a -> (a,a)?
    pub const fn split(&mut self) -> Self {
        SplitRng {
            state: self.next_u64(),
            gamma: {
                let mut z = self.next_u64();
                z ^= z >> 33;
                z = z.wrapping_mul(0x62a9d9ed799705f5);
                z ^= z >> 28;
                z = z.wrapping_mul(0xcb24d0a5c88c35b3);
                z ^= z >> 32;
                z | 1
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference values from prng.di.unimi.it/splitmix64.c with seed 0.
    #[test]
    fn matches_reference() {
        let mut r = SplitRng::new(0);
        assert_eq!(r.next_u64(), 0xE220A8397B1DCDAF);
        assert_eq!(r.next_u64(), 0x6E789E6AA1B965F4);
        assert_eq!(r.next_u64(), 0x06C45D188009454F);
        assert_eq!(r.next_u64(), 0xF88BB8A8724C81EC);
    }
}
