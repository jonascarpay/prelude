use std::marker::PhantomData;

use crate::algebra::{abstract_::VectorSpace, v2, V2};

pub trait Curve {
    type Domain;
    type Codomain;
    fn evaluate(self, x: Self::Domain) -> Self::Codomain;

    fn evaluated(&self, x: Self::Domain) -> Self::Codomain
    where
        Self: Clone,
    {
        self.clone().evaluate(x)
    }

    fn compose<F>(self, f: F) -> Compose<Self, F>
    where
        Self: Sized,
    {
        Compose { g: self, f }
    }

    fn compose_fn<X, Y, F>(self, f: F) -> Compose<Self, FnCurve<X, Y, F>>
    where
        Self: Sized,
    {
        self.compose(FnCurve::new(f))
    }
}

pub trait DifferentiableCurve {
    type Derivative;
    fn derivative(self) -> Self::Derivative;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Compose<G, F> {
    pub g: G,
    pub f: F,
}

impl<G, F> Curve for Compose<G, F>
where
    F: Curve,
    G: Curve<Domain = F::Codomain>,
{
    type Domain = F::Domain;

    type Codomain = G::Codomain;

    fn evaluate(self, x: F::Domain) -> G::Codomain {
        self.g.evaluate(self.f.evaluate(x))
    }
}
impl<G, F> DifferentiableCurve for Compose<G, F>
where
    F: Curve + Clone,
    G: Curve<Domain = F::Codomain>,
    F: DifferentiableCurve,
    G: DifferentiableCurve,
    G::Derivative: VectorSpace<Scalar = F::Derivative>,
{
    type Derivative = Compose<G::Derivative, F>;

    fn derivative(self) -> Self::Derivative {
        // (g o f)' = (g' o f) * f'
        Compose {
            g: self.g.derivative().scale(self.f.clone().derivative()),
            f: self.f,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FnCurve<X, Y, F> {
    pub f: F,
    _phantom: PhantomData<(X, Y)>,
}
impl<X, Y, F> FnCurve<X, Y, F> {
    pub fn new(f: F) -> Self {
        FnCurve {
            f,
            _phantom: PhantomData,
        }
    }
}

impl<X, Y, F: FnOnce(X) -> Y> Curve for FnCurve<X, Y, F> {
    type Domain = X;

    type Codomain = Y;

    fn evaluate(self, x: Self::Domain) -> Self::Codomain {
        (self.f)(x)
    }
}

// Useful for affine mappings
impl<C: Curve> Curve for V2<C> {
    type Domain = V2<C::Domain>;
    type Codomain = V2<C::Codomain>;
    fn evaluate(self, x: Self::Domain) -> Self::Codomain {
        v2(self.x.evaluate(x.x), self.y.evaluate(x.y))
    }
}

// todo: pub trait Surface?

// TODO: .curvature? once wedge product exists?
