use crate::algebra::abstract_::{curve::Compose, Curve, DifferentiableCurve};

#[derive(Clone, Copy)]
pub struct Spline<const N: usize, T> {
    // TODO generalize, with derivative implemented using some kind of functor constraint?
    pub curves: [T; N],
}

pub type UniformSpline<const N: usize, T> = Compose<Spline<N, T>, TruncFract64>;

impl<const N: usize, T> Spline<N, T> {
    pub fn new(curves: [T; N]) -> Self {
        Spline { curves }
    }
    pub fn uniform_f64(curves: [T; N]) -> UniformSpline<N, T>
    where
        T: Curve + Clone,
    {
        Spline::new(curves).compose(TruncFract64)
    }
}

pub struct TruncFract64;

impl Curve for TruncFract64 {
    type Domain = f64;

    type Codomain = (usize, f64);

    fn evaluate(self, x: f64) -> (usize, f64) {
        (x.trunc() as usize, x.fract())
    }
}

impl<const N: usize, T: Curve + Clone> Curve for Spline<N, T> {
    type Domain = (usize, T::Domain);

    type Codomain = T::Codomain;

    fn evaluate(self, (i, x): (usize, T::Domain)) -> Self::Codomain {
        self.curves[i].evaluated(x)
    }
}

impl<const N: usize, T: DifferentiableCurve> DifferentiableCurve for Spline<N, T> {
    type Derivative = Spline<N, T::Derivative>;

    fn derivative(self) -> Self::Derivative {
        Spline {
            curves: self.curves.map(|c| c.derivative()),
        }
    }
}
