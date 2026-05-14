use crate::algebra::rng::split_rng::SplitRng;

#[derive(Clone)]
/// A xoshiro256++ RNG.
/// Very fast, and pretty high-quality.
/// Better performance and statistical properties than SplitRng, but no splitting.
/// For the best of both worlds, draw `FastRng`'s from `SplitRng` using `SplitRng::next_rng`, which is
/// also what `FastRng::new` does.
pub struct FastRng {
    // https://en.wikipedia.org/wiki/Xorshift#xoshiro256++
    // https://prng.di.unimi.it/xoshiro256plusplus.c
    // Fast, decent-quality RNG.
    // Better performance and randomness than splitmix, but no splitting.
    pub(crate) s0: u64,
    pub(crate) s1: u64,
    pub(crate) s2: u64,
    pub(crate) s3: u64,
}

impl FastRng {
    pub const fn new(seed: u64) -> Self {
        SplitRng::new(seed).next_rng()
    }
    pub const fn next_u64(&mut self) -> u64 {
        let res = self
            .s0
            .wrapping_add(self.s3)
            .rotate_left(23)
            .wrapping_add(self.s0);
        let t = self.s1 << 17;
        self.s2 ^= self.s0;
        self.s3 ^= self.s1;
        self.s1 ^= self.s2;
        self.s0 ^= self.s3;
        self.s2 ^= t;
        self.s3 = self.s3.rotate_left(45);

        res
    }

    // Equivalent to 2^128 calls to next_u64()
    // Can be used to split the RNG, but not recursively, since `next_u64` and `jump` commute.
    pub fn jump(&mut self) {
        let consts: [u64; 4] = [
            0x180ec6d33cfd0aba,
            0xd5a61266f0c9392c,
            0xa9582618e03fc9aa,
            0x39abdc4529b1661c,
        ];

        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;

        for c in consts.iter() {
            for b in 0..64 {
                if (c & (1 << b)) != 0 {
                    s0 ^= self.s0;
                    s1 ^= self.s1;
                    s2 ^= self.s2;
                    s3 ^= self.s3;
                }
                self.next_u64();
            }
        }

        self.s0 = s0;
        self.s1 = s1;
        self.s2 = s2;
        self.s3 = s3;
    }
}
