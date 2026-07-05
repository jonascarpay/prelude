pub mod abstract_;
pub mod geometric;
pub mod linear;
pub mod numeric;
pub mod polynomial;

pub use abstract_::{zero, Additive, Ring, VectorSpace};
pub use geometric::vec2::{v2, V2};
pub use linear::affine::{lerp, remap};
