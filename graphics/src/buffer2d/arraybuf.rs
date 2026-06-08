use prelude::algebra::v2;

use crate::buffer2d::buffer2d::{AsMutSlice, AsSlice, Buffer2D, Index2D};

#[derive(Debug, Clone)]
pub struct ArrayBuf<T, const WIDTH: usize, const HEIGHT: usize> {
    arr: [[T; WIDTH]; HEIGHT],
}

impl<T, const WIDTH: usize, const HEIGHT: usize> ArrayBuf<T, WIDTH, HEIGHT> {
    pub const SIZE: Index2D = v2(WIDTH, HEIGHT);

    pub fn filled(v: T) -> Self
    where
        T: Copy,
    {
        ArrayBuf {
            arr: [[v; WIDTH]; HEIGHT],
        }
    }

    pub fn into_buffer(self) -> Buffer2D<Self> {
        Buffer2D::from_packed(self, Self::SIZE)
    }

    pub fn as_buffer(&self) -> Buffer2D<&Self> {
        Buffer2D::from_packed(self, Self::SIZE)
    }

    pub fn as_buffer_mut(&mut self) -> Buffer2D<&mut Self> {
        Buffer2D::from_packed(self, Self::SIZE)
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> AsSlice for ArrayBuf<T, WIDTH, HEIGHT> {
    type Elem = T;
    fn as_slice(&self) -> &[Self::Elem] {
        self.arr.as_flattened()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> AsMutSlice for ArrayBuf<T, WIDTH, HEIGHT> {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self.arr.as_flattened_mut()
    }
}
