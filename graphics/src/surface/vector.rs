use prelude::algebra::{v2, V2};

use crate::{color::srgb::Srgb, format::ppm::write_ppm};

pub struct VectorSurface<T> {
    size: V2<usize>,
    vec: Vec<T>,
}

impl<T> VectorSurface<T> {
    pub fn new(size: V2<usize>) -> Self
    where
        T: Default + Clone,
    {
        Self::replicate(size, T::default())
    }

    pub fn replicate(size: V2<usize>, value: T) -> Self
    where
        T: Clone,
    {
        let n = size.x * size.y; // TODO method?
        VectorSurface {
            size,
            vec: vec![value; n],
        }
    }

    pub fn generate<F: Fn(V2<usize>) -> T>(size: V2<usize>, generator: F) -> Self {
        let n = size.x * size.y; // TODO method?
        VectorSurface {
            size,
            vec: (0..n)
                .map(|i| {
                    let v = v2(i % size.x, i / size.x);
                    generator(v)
                })
                .collect(),
        }
    }
}
impl VectorSurface<Srgb> {
    pub fn write_ppm(&self, path: &str) -> std::io::Result<()> {
        write_ppm(path, self.size, &self.vec)
    }
}
