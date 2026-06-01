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

pub struct PlotLine2D {
    x: isize,
    y: isize,
    x1: isize,
    y1: isize,
    dx: isize,
    dy: isize,
    sx: isize,
    sy: isize,
    err: isize,
    started: bool,
}

impl PlotLine2D {
    pub fn new(start: V2<usize>, end: V2<usize>) -> Self {
        let x = start.x as isize;
        let y = start.y as isize;
        let x1 = end.x as isize;
        let y1 = end.y as isize;
        let dx = (x1 - x).abs();
        let dy = -(y1 - y).abs();
        let sx = if x < x1 { 1 } else { -1 };
        let sy = if y < y1 { 1 } else { -1 };
        let err = dx + dy;
        Self {
            x,
            y,
            x1,
            y1,
            dx,
            dy,
            sx,
            sy,
            err,
            started: false,
        }
    }
}

impl Iterator for PlotLine2D {
    type Item = V2<usize>;

    fn next(&mut self) -> Option<V2<usize>> {
        if self.started {
            if self.x == self.x1 && self.y == self.y1 {
                return None;
            }
            let e2 = 2 * self.err;
            if e2 >= self.dy {
                self.err += self.dy;
                self.x += self.sx;
            }
            if e2 <= self.dx {
                self.err += self.dx;
                self.y += self.sy;
            }
        } else {
            self.started = true;
        }
        Some(v2(self.x as usize, self.y as usize))
    }
}
