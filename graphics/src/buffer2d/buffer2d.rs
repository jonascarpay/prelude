use prelude::algebra::geometric::vec2::v2;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub type Index2D = prelude::algebra::geometric::vec2::V2<usize>;

/// Some storage type that can provide access to its raw data as a slice.
/// Buffer2D provides a 2d abstraction layer on top of this.
pub trait AsSlice {
    type Elem;
    fn as_slice(&self) -> &[Self::Elem];
}

// TODO there is no real point in having this be separate, I think.
pub trait AsMutSlice: AsSlice {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

impl<T: AsSlice + ?Sized> AsSlice for &T {
    type Elem = T::Elem;
    fn as_slice(&self) -> &[T::Elem] {
        (**self).as_slice()
    }
}

impl<T: AsSlice + ?Sized> AsSlice for &mut T {
    type Elem = T::Elem;
    fn as_slice(&self) -> &[T::Elem] {
        (**self).as_slice()
    }
}

impl<T: AsMutSlice + ?Sized> AsMutSlice for &mut T {
    fn as_mut_slice(&mut self) -> &mut [T::Elem] {
        (**self).as_mut_slice()
    }
}

const fn fits_in(small: Index2D, large: Index2D) -> bool {
    small.x <= large.x && small.y <= large.y
}

#[derive(Debug, Clone, Copy)]
pub struct Buffer2D<T> {
    pub(crate) parent: T,
    pub(crate) parent_stride: usize,
    pub(crate) offset: Index2D,
    pub(crate) size: Index2D,
}

impl<T> Buffer2D<T> {
    pub const fn from_packed(parent: T, size: Index2D) -> Self {
        Buffer2D {
            parent,
            parent_stride: size.x,
            offset: v2(0, 0),
            size,
        }
    }

    pub const fn size(&self) -> Index2D {
        self.size
    }

    fn row_start(&self, y: usize) -> usize {
        (self.offset.y + y) * self.parent_stride + self.offset.x
    }

    pub fn slice(self, offset: Index2D, size: Index2D) -> Option<Self> {
        if fits_in(offset + size, self.size) {
            Some(Buffer2D {
                offset: self.offset + offset,
                size,
                ..self
            })
        } else {
            None
        }
    }
}

impl<T: AsSlice> Buffer2D<T> {
    pub fn from_packed_checked(parent: T, size: Index2D) -> Option<Self> {
        (parent.as_slice().len() == size.x * size.y).then(|| Self::from_packed(parent, size))
    }

    fn row(&self, y: usize) -> &[T::Elem] {
        let start = self.row_start(y);
        &self.parent.as_slice()[start..start + self.size.x]
    }

    pub fn get(&self, ix: Index2D) -> &T::Elem {
        &self.row(ix.y)[ix.x]
    }
}

impl<T: AsMutSlice> Buffer2D<T> {
    fn row_mut(&mut self, y: usize) -> &mut [T::Elem] {
        let start = self.row_start(y);
        let width = self.size.x;
        &mut self.parent.as_mut_slice()[start..start + width]
    }

    pub fn get_mut(&mut self, ix: Index2D) -> &mut T::Elem {
        &mut self.row_mut(ix.y)[ix.x]
    }

    pub fn fill(&mut self, value: T::Elem)
    where
        T::Elem: Clone,
    {
        for y in 0..self.size.y {
            self.row_mut(y).fill(value.clone());
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn generate<F>(&mut self, mut f: F)
    where
        F: FnMut(Index2D) -> T::Elem,
    {
        for y in 0..self.size.y {
            let row = self.row_mut(y);
            for x in 0..row.len() {
                row[x] = f(v2(x, y));
            }
        }
    }

    pub fn blit<S>(&mut self, src: &Buffer2D<S>, at: Index2D) -> Option<()>
    where
        S: AsSlice<Elem = T::Elem>,
        T::Elem: Copy,
    {
        let size = src.size;
        if !fits_in(at + size, self.size) {
            return None;
        }
        for y in 0..size.y {
            self.row_mut(at.y + y)[at.x..at.x + size.x].copy_from_slice(src.row(y));
        }
        Some(())
    }
}

impl<'a, 'b, D: HasDisplayHandle, W: HasWindowHandle>
    Buffer2D<&'a mut softbuffer::Buffer<'b, D, W>>
{
    pub fn from_softbuffer(buf: &'a mut softbuffer::Buffer<'b, D, W>) -> Self {
        let w = buf.width().get() as usize;
        let h = buf.height().get() as usize;
        Buffer2D::from_packed(buf, v2(w, h))
    }
}

impl<D, W> AsSlice for softbuffer::Buffer<'_, D, W>
where
    D: HasDisplayHandle,
    W: HasWindowHandle,
{
    type Elem = u32;
    fn as_slice(&self) -> &[u32] {
        self
    }
}
impl<D, W> AsMutSlice for softbuffer::Buffer<'_, D, W>
where
    D: HasDisplayHandle,
    W: HasWindowHandle,
{
    fn as_mut_slice(&mut self) -> &mut [u32] {
        self
    }
}
