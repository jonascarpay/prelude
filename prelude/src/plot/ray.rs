use crate::algebra::{v2, Ring, V2};

pub fn plot_ray2d<T: Ring + Ord + Copy>(start: V2<T>, through: V2<T>) -> Ray2D<T> {
    Ray2D::new(start, through)
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
