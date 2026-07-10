pub trait Semigroup {
    /// An associative operation
    fn compose(self, rhs: Self) -> Self;
}

pub trait Monoid: Semigroup {
    /// The identity element for `compose`
    // TODO: constant
    fn identity() -> Self;
}

pub trait Group: Monoid {
    /// An inverse for `compose`
    fn inverse(self) -> Self;
}

// TODO IntoIterator?
pub fn iter_concat<T: Monoid, I: Iterator<Item = T>>(iter: I) -> T {
    iter.fold(T::identity(), T::compose)
}

/// Does not `compose` 0 and the first element.
/// Tends to optimizer better for small fixed-size iterators, worse for large/dynamic ones.
pub fn iter_concat_reduce<T: Monoid, I: Iterator<Item = T>>(iter: I) -> T {
    iter.reduce(T::compose).unwrap_or(T::identity())
}
