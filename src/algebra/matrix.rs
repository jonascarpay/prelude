use std::ops::Mul;

use crate::algebra::{
    abstract_::{additive::iter_sum, Additive, Ring},
    geometric::vec2::V2,
};

pub struct Matrix<T, const R: usize, const C: usize> {
    arr: [[T; C]; R],
}

impl<T: Clone, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn transpose(self) -> Matrix<T, C, R> {
        use std::array::from_fn;
        Matrix {
            arr: from_fn(|r| from_fn(|c| self.arr[c][r].clone())),
        }
    }
}

impl<T, const R: usize, const C: usize> std::ops::Index<V2<usize>> for Matrix<T, R, C> {
    type Output = T;
    fn index(&self, V2 { x: c, y: r }: V2<usize>) -> &Self::Output {
        &self.arr[r][c]
    }
}

impl<T, const R: usize, const C: usize> std::ops::Index<usize> for Matrix<T, R, C> {
    type Output = [T; C];
    fn index(&self, r: usize) -> &Self::Output {
        &self.arr[r]
    }
}

#[inline]
fn generic_matmul<
    A,
    B,
    C: Additive,
    F: Fn(&A, &B) -> C,
    const ROW: usize,
    const HID: usize,
    const COL: usize,
>(
    a: Matrix<A, ROW, HID>,
    b: Matrix<B, HID, COL>,
    f: F,
) -> Matrix<C, ROW, COL> {
    use std::array::from_fn;
    Matrix {
        arr: from_fn(|r| {
            from_fn(|c| {
                let hs: [C; HID] = from_fn(|h| f(&a[r][h], &b[h][c]));
                iter_sum(hs.into_iter())
            })
        }),
    }
}

impl<T: Ring, const R: usize, const H: usize, const C: usize> Mul<Matrix<T, H, C>>
    for Matrix<T, R, H>
{
    type Output = Matrix<T, R, C>;
    fn mul(self, rhs: Matrix<T, H, C>) -> Self::Output {
        generic_matmul(self, rhs, |a, b| b.clone().mult(a.clone()))
    }
}

// TODO a method/newtype for doing vector space `scale` instead of ring `mult`
// TODO VectorSpace
// TODO nxn Ring
