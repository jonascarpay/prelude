use crate::{
    algebra::{
        abstract_::{additive::iter_sum, Ring},
        V2,
    },
    plot::{
        itertools::{StopsAt, StopsBefore},
        ray::{plot_ray2d, Ray2D},
    },
};

/// Checked to be as optimizer-friendly as a hand-rolled implementation.
pub fn plot_line2d<T: Ring + Ord + Copy>(start: V2<T>, end: V2<T>) -> StopsAt<Ray2D<T>> {
    StopsAt::new(plot_ray2d(start, end), end)
}

pub fn plot_open_line2d<T: Ring + Ord + Copy>(start: V2<T>, end: V2<T>) -> StopsBefore<Ray2D<T>> {
    StopsBefore::new(plot_ray2d(start, end), end)
}

#[inline(never)]
pub fn reference(start: V2<usize>, end: V2<usize>) -> V2<usize> {
    iter_sum(plot_line2d(start, end))
}

#[cfg(test)]
mod tests {
    use crate::algebra::v2;

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
