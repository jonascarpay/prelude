use crate::random::rng::Rng;

pub trait Distribution {
    type Output;
    fn sample<R: Rng>(&self, rng: &mut R) -> Self::Output;

    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> B,
    {
        Map { dist: self, f }
    }
}

pub trait DistributionExt<D: Distribution> {
    fn draw(&mut self, dist: D) -> D::Output;
}

impl<T: Rng, D: Distribution> DistributionExt<D> for T {
    fn draw(&mut self, dist: D) -> D::Output {
        dist.sample(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Map<D, F> {
    dist: D,
    f: F,
}

impl<D, F, B> Distribution for Map<D, F>
where
    D: Distribution,
    F: Fn(D::Output) -> B,
{
    type Output = B;
    fn sample<R: Rng>(&self, rng: &mut R) -> B {
        (self.f)(self.dist.sample(rng))
    }
}
