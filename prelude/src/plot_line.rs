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
    iter_sum(plot_line(start, end))
}

pub fn plot_line<T: Ring + Ord + Copy>(start: V2<T>, end: V2<T>) -> EndsAt<Ray2D<T>> {
    EndsAt::new(Ray2D::new(start, end), end)
}

pub fn plot_open_line<T: Ring + Ord + Copy>(
    start: V2<T>,
    end: V2<T>,
) -> EndsBefore<Ray2D<T>> {
    EndsBefore::new(Ray2D::new(start, end), end)
}

pub struct Ray2D<T> {
    pos: V2<T>,
    s: V2<T>,
    err: T,
    threshold: T,
    step: T,
    x_major: bool,
    started: bool,
}

impl<T: Ring + Ord + Copy> Ray2D<T> {
    pub fn new(start: V2<T>, through: V2<T>) -> Self {
        let one = T::one();
        let (dx, sx) = if start.x < through.x {
            (through.x.minus(start.x), one)
        } else {
            (start.x.minus(through.x), one.negate())
        };
        let (dy, sy) = if start.y < through.y {
            (through.y.minus(start.y), one)
        } else {
            (start.y.minus(through.y), one.negate())
        };
        let x_major = dx >= dy;
        let (major, minor) = if x_major { (dx, dy) } else { (dy, dx) };
        let threshold = major.plus(major);
        let step = minor.plus(minor);
        Self {
            pos: start,
            s: v2(sx, sy),
            err: major,
            threshold,
            step,
            x_major,
            started: false,
        }
    }
}

impl<T: Ring + Ord + Copy> Iterator for Ray2D<T> {
    type Item = V2<T>;

    fn next(&mut self) -> Option<V2<T>> {
        if self.started {
            self.err = self.err.plus(self.step);
            let minor_step = self.err >= self.threshold;
            if minor_step {
                self.err = self.err.minus(self.threshold);
            }
            if self.x_major {
                self.pos.x = self.pos.x.plus(self.s.x);
                if minor_step {
                    self.pos.y = self.pos.y.plus(self.s.y);
                }
            } else {
                self.pos.y = self.pos.y.plus(self.s.y);
                if minor_step {
                    self.pos.x = self.pos.x.plus(self.s.x);
                }
            }
        } else {
            self.started = true;
        }
        Some(self.pos)
    }
}

pub struct EndsAt<I: Iterator> {
    iter: I,
    end: I::Item,
    done: bool,
}

impl<I: Iterator> EndsAt<I> {
    pub fn new(iter: I, end: I::Item) -> Self {
        Self {
            iter,
            end,
            done: false,
        }
    }
}

impl<I> Iterator for EndsAt<I>
where
    I: Iterator,
    I::Item: PartialEq + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.done {
            return None;
        }
        let item = self.iter.next()?;
        if item == self.end {
            self.done = true;
        }
        Some(item)
    }
}

pub struct EndsBefore<I: Iterator> {
    iter: I,
    end: I::Item,
    done: bool,
}

impl<I: Iterator> EndsBefore<I> {
    pub fn new(iter: I, end: I::Item) -> Self {
        Self {
            iter,
            end,
            done: false,
        }
    }
}

impl<I> Iterator for EndsBefore<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.done {
            return None;
        }
        let item = self.iter.next()?;
        if item == self.end {
            self.done = true;
            None
        } else {
            Some(item)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn vec() -> impl Strategy<Value = V2<usize>> {
        (0usize..256, 0usize..256).prop_map(|(x, y)| v2(x, y))
    }

    fn abs_diff(a: usize, b: usize) -> usize {
        a.max(b) - a.min(b)
    }

    proptest! {
        #[test]
        fn first_is_start(start in vec(), end in vec()) {
            let first = plot_line(start, end).next();
            prop_assert_eq!(first, Some(start));
        }

        #[test]
        fn last_is_end(start in vec(), end in vec()) {
            let last = plot_line(start, end).last();
            prop_assert_eq!(last, Some(end));
        }

        #[test]
        fn length_matches_max_delta(start in vec(), end in vec()) {
            let count = plot_line(start, end).count();
            let dx = abs_diff(end.x, start.x);
            let dy = abs_diff(end.y, start.y);
            prop_assert_eq!(count, dx.max(dy) + 1);
        }

        #[test]
        fn no_gaps(start in vec(), end in vec()) {
            let points: Vec<_> = plot_line(start, end).collect();
            for w in points.windows(2) {
                let dx = abs_diff(w[1].x, w[0].x);
                let dy = abs_diff(w[1].y, w[0].y);
                prop_assert!(dx <= 1 && dy <= 1, "step too large: {:?} -> {:?}", w[0], w[1]);
                prop_assert!(dx + dy > 0, "duplicate point: {:?}", w[0]);
            }
        }
    }
}
