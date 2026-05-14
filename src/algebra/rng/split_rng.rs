use crate::algebra::rng::fast_rng::FastRng;

#[derive(Clone)]
/// A splittable RNG, based on SplitMix64.
/// Not quite as fast or random as FastRng, but can be split.
/// For the best of both worlds, draw `FastRng`'s from `SplitRng` using `SplitRng::next_rng`, which
/// is also what `FastRng::new` does.
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
        const fn mix_gamma(mut z: u64) -> u64 {
            z ^= z >> 33;
            z = z.wrapping_mul(0x62a9d9ed799705f5);
            z ^= z >> 28;
            z = z.wrapping_mul(0xcb24d0a5c88c35b3);
            z ^= z >> 32;
            z | 1
        }

        SplitRng {
            state: self.next_u64(),
            gamma: mix_gamma(self.next_u64()),
        }
    }
}
