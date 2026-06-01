use crate::algebra::{
    abstract_::{additive::iter_sum, Additive},
    v2, V2,
};

#[inline(never)]
pub fn bresenham_sum_reference(start: V2<usize>, end: V2<usize>) -> V2<usize> {
    let mut x = start.x as isize;
    let mut y = start.y as isize;
    let x1 = end.x as isize;
    let y1 = end.y as isize;
    let dx = (x1 - x).abs();
    let dy = -(y1 - y).abs();
    let sx = if x < x1 { 1 } else { -1 };
    let sy = if y < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut out = V2::zero();
    loop {
        out.incr_by(v2(x as usize, y as usize));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    out
}

#[inline(never)]
pub fn bresenham_sum_ours(start: V2<usize>, end: V2<usize>) -> V2<usize> {
    iter_sum(PlotLine2D::new(start, end))
}

pub struct PlotLine2D;

impl PlotLine2D {
    pub fn new(start: V2<usize>, end: V2<usize>) -> Self {
        todo!()
    }
}

impl Iterator for PlotLine2D {
    type Item = V2<usize>;

    fn next(&mut self) -> Option<V2<usize>> {
        todo!()
    }
}
