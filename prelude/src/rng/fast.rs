use crate::rng::split::SplitRng;

#[derive(Clone, Debug)]
/// Fast and high-quality RNG based on xoshiro256++.
pub struct FastRng {
    // https://en.wikipedia.org/wiki/Xorshift#xoshiro256++
    // https://prng.di.unimi.it/xoshiro256plusplus.c
    pub(super) s0: u64,
    pub(super) s1: u64,
    pub(super) s2: u64,
    pub(super) s3: u64,
}

impl Default for FastRng {
    fn default() -> Self {
        FastRng::new(9)
    }
}

impl FastRng {
    pub const fn new(seed: u64) -> Self {
        SplitRng::new(seed).next_rng()
    }
    pub const fn next_u64(&mut self) -> u64 {
        let res = self.s0.wrapping_add(self.s3).rotate_left(23).wrapping_add(self.s0);
        let t = self.s1 << 17;
        self.s2 ^= self.s0;
        self.s3 ^= self.s1;
        self.s1 ^= self.s2;
        self.s0 ^= self.s3;
        self.s2 ^= t;
        self.s3 = self.s3.rotate_left(45);

        res
    }

    /// Equivalent to 2^128 calls to next_u64().
    /// Can be used to as a quick way to get independent sequences, but not a true split.
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

#[cfg(test)]
mod tests {
    use super::*;

    const REF: FastRng = FastRng {
        s0: 1,
        s1: 2,
        s2: 3,
        s3: 4,
    };

    // Reference values from prng.di.unimi.it/xoshiro256plusplus.c
    #[test]
    fn matches_reference() {
        let mut r = REF.clone();
        assert_eq!(r.next_u64(), 0x0000000002800001);
        assert_eq!(r.next_u64(), 0x0000000003800067);
        assert_eq!(r.next_u64(), 0x000CC00003800067);
        assert_eq!(r.next_u64(), 0x000CC201994400B2);
    }

    #[test]
    fn jump_matches_reference() {
        let mut r = REF.clone();
        r.jump();
        assert_eq!(r.s0, 0x8C7A153956B5F3D1);
        assert_eq!(r.s1, 0x701F1A713401D85E);
        assert_eq!(r.s2, 0x6527F66A65469085);
        assert_eq!(r.s3, 0x8386B786C4408050);
    }

    #[test]
    fn jump_next_commute() {
        let r = FastRng::new(0);

        let mut a = r.clone();
        a.jump();
        a.next_u64();

        let mut b = r.clone();
        b.next_u64();
        b.jump();

        assert_eq!(a.next_u64(), b.next_u64());
    }
}
