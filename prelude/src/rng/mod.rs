pub mod fast;
pub mod split;

pub trait Rng {
    fn next_u64(&mut self) -> u64;
}

impl Rng for fast::FastRng {
    fn next_u64(&mut self) -> u64 {
        self.next_u64()
    }
}

impl Rng for split::SplitRng {
    fn next_u64(&mut self) -> u64 {
        self.next_u64()
    }
}

#[cfg(test)]
mod tests {

    use crate::rng::{fast::FastRng, split::SplitRng, Rng};

    #[test]
    fn fast_doesnt_spin() {
        let mut rng = FastRng::new(0);
        let _ = <FastRng as Rng>::next_u64(&mut rng);
    }

    #[test]
    fn split_doesnt_spin() {
        let mut rng = SplitRng::new(0);
        let _ = <SplitRng as Rng>::next_u64(&mut rng);
    }
}
