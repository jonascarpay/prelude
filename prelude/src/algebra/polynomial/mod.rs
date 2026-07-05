pub mod dense;
pub mod even;
pub mod interpolations;
pub mod odd;

pub use dense::{Cubic, Linear, Quadratic, Quartic, Quintic};
pub use even::{EvenQuadratic, EvenQuartic};
pub use interpolations::{bezier2, bezier3, unit_hermite3, unit_hermite5};
pub use odd::{OddCubic, OddQuintic};
