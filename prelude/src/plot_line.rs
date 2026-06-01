use crate::algebra::{
    abstract_::{additive::iter_sum, Additive, Ring},
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
    let start = v2(start.x as isize, start.y as isize);
    let end = v2(end.x as isize, end.y as isize);
    let sum: V2<isize> = iter_sum(PlotLine2D::new(start, end));
    v2(sum.x as usize, sum.y as usize)
}

pub struct PlotLine2D<T> {
    pos: V2<T>,
    end: V2<T>,
    d: V2<T>,
    s: V2<T>,
    err: T,
    started: bool,
}

impl<T: Ring + Ord + Copy> PlotLine2D<T> {
    pub fn new(start: V2<T>, end: V2<T>) -> Self {
        let one = T::one();
        let dx_signed = end.x.minus(start.x);
        let dy_signed = end.y.minus(start.y);
        let dx = if dx_signed < T::zero() {
            dx_signed.negate()
        } else {
            dx_signed
        };
        let dy = if dy_signed < T::zero() {
            dy_signed
        } else {
            dy_signed.negate()
        };
        let sx = if start.x < end.x { one } else { one.negate() };
        let sy = if start.y < end.y { one } else { one.negate() };
        let err = dx.plus(dy);
        Self {
            pos: start,
            end,
            d: v2(dx, dy),
            s: v2(sx, sy),
            err,
            started: false,
        }
    }
}

impl<T: Ring + Ord + Copy> Iterator for PlotLine2D<T> {
    type Item = V2<T>;

    fn next(&mut self) -> Option<V2<T>> {
        if self.started {
            if self.pos == self.end {
                return None;
            }
            let e2 = self.err.plus(self.err);
            if e2 >= self.d.y {
                self.err = self.err.plus(self.d.y);
                self.pos.x = self.pos.x.plus(self.s.x);
            }
            if e2 <= self.d.x {
                self.err = self.err.plus(self.d.x);
                self.pos.y = self.pos.y.plus(self.s.y);
            }
        } else {
            self.started = true;
        }
        Some(self.pos)
    }
}
