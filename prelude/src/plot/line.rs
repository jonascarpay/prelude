use crate::{
    algebra::{
        abstract_::{additive::iter_sum, Additive, Ring},
        v2, V2,
    },
    plot::{
        itertools::{StopsAt, StopsBefore},
        ray::{plot_ray2d, Ray2D},
    },
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
    iter_sum(plot_line2d(start, end))
}

pub fn plot_line2d<T: Ring + Ord + Copy>(start: V2<T>, end: V2<T>) -> StopsAt<Ray2D<T>> {
    StopsAt::new(plot_ray2d(start, end), end)
}

pub fn plot_open_line2d<T: Ring + Ord + Copy>(start: V2<T>, end: V2<T>) -> StopsBefore<Ray2D<T>> {
    StopsBefore::new(plot_ray2d(start, end), end)
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
            let first = plot_line2d(start, end).next();
            prop_assert_eq!(first, Some(start));
        }

        #[test]
        fn last_is_end(start in vec(), end in vec()) {
            let last = plot_line2d(start, end).last();
            prop_assert_eq!(last, Some(end));
        }

        #[test]
        fn length_matches_max_delta(start in vec(), end in vec()) {
            let count = plot_line2d(start, end).count();
            let dx = abs_diff(end.x, start.x);
            let dy = abs_diff(end.y, start.y);
            prop_assert_eq!(count, dx.max(dy) + 1);
        }

        #[test]
        fn no_gaps(start in vec(), end in vec()) {
            let points: Vec<_> = plot_line2d(start, end).collect();
            for w in points.windows(2) {
                let dx = abs_diff(w[1].x, w[0].x);
                let dy = abs_diff(w[1].y, w[0].y);
                prop_assert!(dx <= 1 && dy <= 1, "step too large: {:?} -> {:?}", w[0], w[1]);
                prop_assert!(dx + dy > 0, "duplicate point: {:?}", w[0]);
            }
        }
    }
}
