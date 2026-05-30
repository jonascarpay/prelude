use prelude::algebra::geometric::vec2::v2;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub type Index2D = prelude::algebra::geometric::vec2::V2<usize>;

pub trait Buffer {
    type Elem;
    fn as_slice(&self) -> &[Self::Elem];
}

pub trait BufferMut: Buffer {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

impl<T: Buffer + ?Sized> Buffer for &mut T {
    type Elem = T::Elem;
    fn as_slice(&self) -> &[T::Elem] {
        (**self).as_slice()
    }
}

impl<T: BufferMut + ?Sized> BufferMut for &mut T {
    fn as_mut_slice(&mut self) -> &mut [T::Elem] {
        (**self).as_mut_slice()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Buffer2DView<T> {
    parent: T,
    parent_stride: usize,
    offset: Index2D,
    size: Index2D,
}

fn fits_in(small: Index2D, large: Index2D) -> bool {
    small.x <= large.x && small.y <= large.y
}

impl<T> Buffer2DView<T> {
    pub fn size(&self) -> Index2D {
        self.size
    }

    fn row_start(&self, y: usize) -> usize {
        (self.offset.y + y) * self.parent_stride + self.offset.x
    }

    pub fn slice(self, offset: Index2D, size: Index2D) -> Option<Self> {
        if fits_in(offset + size, self.size) {
            Some(Buffer2DView {
                offset: self.offset + offset,
                size,
                ..self
            })
        } else {
            None
        }
    }
}

impl<T: Buffer> Buffer2DView<T> {
    fn row(&self, y: usize) -> &[T::Elem] {
        let start = self.row_start(y);
        &self.parent.as_slice()[start..start + self.size.x]
    }

    pub fn get(&self, ix: Index2D) -> &T::Elem {
        &self.row(ix.y)[ix.x]
    }
}

impl<T: BufferMut> Buffer2DView<T> {
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

    pub fn blit<S>(&mut self, src: &Buffer2DView<S>, at: Index2D) -> Option<()>
    where
        S: Buffer<Elem = T::Elem>,
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

impl<'a, 'b, D: HasDisplayHandle, W: HasWindowHandle> Buffer2DView<&'a mut softbuffer::Buffer<'b, D, W>> {
    pub fn from_softbuffer(buf: &'a mut softbuffer::Buffer<'b, D, W>) -> Self {
        let w = buf.width().get() as usize;
        let h = buf.height().get() as usize;
        Buffer2DView {
            parent: buf,
            parent_stride: w,
            offset: v2(0, 0),
            size: v2(w, h),
        }
    }
}

impl<D, W> Buffer for softbuffer::Buffer<'_, D, W>
where
    D: HasDisplayHandle,
    W: HasWindowHandle,
{
    type Elem = u32;
    fn as_slice(&self) -> &[u32] {
        self
    }
}
impl<D, W> BufferMut for softbuffer::Buffer<'_, D, W>
where
    D: HasDisplayHandle,
    W: HasWindowHandle,
{
    fn as_mut_slice(&mut self) -> &mut [u32] {
        self
    }
}
