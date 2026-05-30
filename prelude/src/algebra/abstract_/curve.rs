pub trait Curve {
    type Domain;
    type Codomain;
    fn evaluate(self, x: Self::Domain) -> Self::Codomain;
}

pub trait DifferentiableCurve: Curve {
    type Derivative;
    fn derivative(self) -> Self::Derivative;
}

// todo: pub trait Surface
