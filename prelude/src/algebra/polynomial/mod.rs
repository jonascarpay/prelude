pub mod dense;
pub mod interpolations;

pub use dense::{Cubic, Linear, Quadratic, Quartic, Quintic};
pub use interpolations::{bezier2, bezier3, unit_hermite3, unit_hermite5};
