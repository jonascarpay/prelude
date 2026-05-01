pub trait Curve {
    type Domain;
    type Range;
    fn evaluate(self, x: Self::Domain) -> Self::Range;
}

pub trait DifferentiableCurve: Curve {
    type Derivative;
}

// todo: pub trait Surface
