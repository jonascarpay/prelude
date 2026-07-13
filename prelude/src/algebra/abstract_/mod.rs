pub mod additive;
pub mod euclidean_ring;
pub mod field;
pub mod functor;
pub mod group;
pub mod inner_product_space;
pub mod module;
pub mod real;
pub mod ring;
pub mod vector_space;

pub use {
    additive::{zero, Additive},
    euclidean_ring::EuclideanRing,
    functor::Functor,
    group::{Group, Monoid, Semigroup},
    inner_product_space::InnerProductSpace,
    module::Module,
    ring::Ring,
    vector_space::VectorSpace,
};

pub use crate::{
    impl_additive_ops, impl_module_div, impl_module_mul, impl_ring_ops, impl_vector_space_div,
    impl_vector_space_mul, impl_vector_space_ops,
};
