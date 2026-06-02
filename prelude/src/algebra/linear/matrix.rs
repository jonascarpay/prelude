use std::{
    array::{from_fn, repeat},
    ops::Mul,
};

use crate::algebra::{
    abstract_::{additive::iter_sum_reduce, Additive, Ring, VectorSpace},
    geometric::{vec2::V2, vec3::V3},
};

/// Alias for `Matrix::from_rows`.
pub const fn mat<T, const R: usize, const C: usize>(rows: [[T; C]; R]) -> Matrix<T, R, C> {
    Matrix { rows }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix<T, const R: usize, const C: usize> {
    rows: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn from_rows(rows: [[T; C]; R]) -> Self {
        Self { rows }
    }
}

impl<T: Ring, const N: usize> Matrix<T, N, N> {
    pub fn identity() -> Self {
        Matrix {
            rows: from_fn(|r| from_fn(|c| if r == c { T::one() } else { T::zero() })),
        }
    }
}

impl<T: Additive, const R: usize, const C: usize> Additive for Matrix<T, R, C> {
    fn plus(self, rhs: Self) -> Self {
        Matrix {
            rows: self.rows.plus(rhs.rows),
        }
    }

    fn zero() -> Self {
        Matrix {
            rows: repeat(repeat(T::zero())),
        }
    }

    fn negate(self) -> Self {
        Matrix {
            rows: self.rows.negate(),
        }
    }
}
impl<T: Ring, const N: usize> Ring for Matrix<T, N, N> {
    fn mult(self, rhs: Self) -> Self {
        generic_matmul(self, rhs, |a, b| T::mult(a.clone(), b.clone()))
    }

    fn from_integer(i: isize) -> Self {
        let i = T::from_integer(i);
        Matrix {
            rows: from_fn(|r| from_fn(|c| if r == c { i.clone() } else { T::zero() })),
        }
    }
}

impl<T: Ring, const R: usize, const C: usize> VectorSpace for Matrix<T, R, C> {
    type Scalar = T;

    fn scale(self, c: Self::Scalar) -> Self {
        Matrix {
            rows: self.rows.map(|r| r.map(|x| c.clone().mult(x))),
        }
    }
}

impl<T: Clone, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn transpose(self) -> Matrix<T, C, R> {
        use std::array::from_fn;
        Matrix {
            rows: from_fn(|r| from_fn(|c| self.rows[c][r].clone())),
        }
    }
}

impl<T, const R: usize, const C: usize> std::ops::Index<V2<usize>> for Matrix<T, R, C> {
    type Output = T;
    fn index(&self, V2 { x: c, y: r }: V2<usize>) -> &Self::Output {
        &self.rows[r][c]
    }
}

impl<T, const R: usize, const C: usize> std::ops::Index<usize> for Matrix<T, R, C> {
    type Output = [T; C];
    fn index(&self, r: usize) -> &Self::Output {
        &self.rows[r]
    }
}

#[inline]
fn generic_vecmul<A, B, C: Additive, F: Fn(&A, &B) -> C, const ROW: usize, const COL: usize>(
    a: Matrix<A, ROW, COL>,
    b: [B; COL],
    f: F,
) -> [C; ROW] {
    from_fn(|r| iter_sum_reduce(a[r].iter().zip(b.iter()).map(|(a, b)| f(a, b))))
}

#[inline]
fn generic_matmul<A, B, C: Additive, F: Fn(&A, &B) -> C, const ROW: usize, const HID: usize, const COL: usize>(
    a: Matrix<A, ROW, HID>,
    b: Matrix<B, HID, COL>,
    f: F,
) -> Matrix<C, ROW, COL> {
    Matrix {
        rows: from_fn(|r| from_fn(|c| iter_sum_reduce((0..HID).map(|h| f(&a[r][h], &b[h][c]))))),
    }
}

impl<T: Ring, const R: usize, const H: usize, const C: usize> Mul<Matrix<T, H, C>> for Matrix<T, R, H> {
    type Output = Matrix<T, R, C>;
    fn mul(self, rhs: Matrix<T, H, C>) -> Self::Output {
        generic_matmul(self, rhs, |a, b| a.clone().mult(b.clone()))
    }
}

impl<T: Ring, const R: usize, const C: usize> Mul<[T; C]> for Matrix<T, R, C> {
    type Output = [T; R];
    fn mul(self, rhs: [T; C]) -> Self::Output {
        generic_vecmul(self, rhs, |a, b| a.clone().mult(b.clone()))
    }
}

impl<T: Ring> Mul<V2<T>> for Matrix<T, 2, 2> {
    type Output = V2<T>;
    fn mul(self, rhs: V2<T>) -> V2<T> {
        V2::unpack(generic_vecmul(self, rhs.pack(), |a, b| a.clone().mult(b.clone())))
    }
}

impl<T: Ring> Mul<V3<T>> for Matrix<T, 3, 3> {
    type Output = V3<T>;
    fn mul(self, rhs: V3<T>) -> V3<T> {
        V3::unpack(generic_vecmul(self, rhs.pack(), |a, b| a.clone().mult(b.clone())))
    }
}

// TODO a method/newtype for doing vector space `scale` instead of ring `mult`

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::geometric::vec2::v2;
    use proptest::prelude::*;

    #[test]
    fn matvec_known_answer() {
        let m = mat([[1, 2], [3, 4]]);
        assert_eq!(m * v2(5, 6), v2(17, 39));
    }

    #[test]
    fn matmul_known_answer() {
        let a = mat([[1, 2], [3, 4]]);
        let b = mat([[5, 6], [7, 8]]);
        assert_eq!(a * b, mat([[19, 22], [43, 50]]));
    }

    proptest! {
        #[test]
        fn matvec_composes_with_matmul(a: [[i32; 2]; 2], b: [[i32; 2]; 2], v: (i32, i32)) {
            let (a, b, v) = (mat(a), mat(b), v2(v.0, v.1));
            prop_assert_eq!((a * b) * v, a * (b * v));
        }

        #[test]
        fn matvec_is_additive(m: [[i32; 2]; 2], u: (i32, i32), w: (i32, i32)) {
            let m = mat(m);
            let (u, w) = (v2(u.0, u.1), v2(w.0, w.1));
            prop_assert_eq!(m * (u + w), (m * u) + (m * w));
        }

        #[test]
        fn identity_is_neutral(v: (i32, i32)) {
            prop_assert_eq!(Matrix::identity() * v2(v.0, v.1), v2(v.0, v.1));
        }
    }
}
