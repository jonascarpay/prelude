use super::additive::Additive;
use super::ring::Ring;

pub trait VectorSpace: Additive
where
    Self::Over: Ring,
{
    type Over;

    /// Scalar multiplication such that
    ///   - v.scale(a).scale(b) = v.scale(a.mult(b))
    ///   - v.scale(one()) = v
    ///   - v.plus(u).scale(a) = v.scale(a).plus(u.scale(a))
    ///   - v.scale(a.plus(b)) = v.scale(a).plus(v.scale(b))
    fn scale(self, c: Self::Over) -> Self;
}

impl<R, A, B> VectorSpace for (A, B)
where
    R: Ring + Clone,
    A: VectorSpace<Over = R>,
    B: VectorSpace<Over = R>,
{
    type Over = R;

    fn scale(self, c: Self::Over) -> Self {
        let (a, b) = self;
        (a.scale(c.clone()), b.scale(c))
    }
}

impl<R, A, B, C> VectorSpace for (A, B, C)
where
    R: Ring + Clone,
    A: VectorSpace<Over = R>,
    B: VectorSpace<Over = R>,
    C: VectorSpace<Over = R>,
{
    type Over = R;

    fn scale(self, c: Self::Over) -> Self {
        let (a, b, cc) = self;
        (a.scale(c.clone()), b.scale(c.clone()), cc.scale(c))
    }
}

impl<R, A, B, C, D> VectorSpace for (A, B, C, D)
where
    R: Ring + Clone,
    A: VectorSpace<Over = R>,
    B: VectorSpace<Over = R>,
    C: VectorSpace<Over = R>,
    D: VectorSpace<Over = R>,
{
    type Over = R;

    fn scale(self, s: Self::Over) -> Self {
        let (a, b, c, d) = self;
        (
            a.scale(s.clone()),
            b.scale(s.clone()),
            c.scale(s.clone()),
            d.scale(s),
        )
    }
}

impl<R, T, const N: usize> VectorSpace for [T; N]
where
    R: Ring + Clone,
    T: VectorSpace<Over = R>,
{
    type Over = R;

    fn scale(self, c: Self::Over) -> Self {
        self.map(|x| x.scale(c.clone()))
    }
}
