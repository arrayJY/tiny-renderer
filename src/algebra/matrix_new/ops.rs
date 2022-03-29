use super::Matrix;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign, Index, IndexMut};

impl_basic_ops!(Add, add, +, Matrix);
impl_basic_ops!(Sub, sub, -, Matrix);
impl_assign_ops!(AddAssign, add_assign, +=, Matrix);
impl_assign_ops!(SubAssign, sub_assign, -=, Matrix);
impl_scalar_ops!(Mul, mul, *, Matrix);
impl_scalar_ops!(Div, div, /, Matrix);

impl<const N: usize> Mul<Matrix<N>> for Matrix<N> {
    type Output = Matrix<N>;
    fn mul(self, rhs: Matrix<N>) -> Self::Output {
        let mut m = Self::Output::new();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    unsafe {
                        let v = self.get_unchecked(i, k) * rhs.get_unchecked(k, j);
                        *m.get_unchecked_mut(i, j) += v;
                    }
                }
            }
        }
        m
    }
}

impl<'a, 'b, const N: usize> Mul<&'a Matrix<N>> for &'b Matrix<N>
where
    'a: 'b,
{
    type Output = Matrix<N>;
    fn mul(self, rhs: &'a Matrix<N>) -> Self::Output {
        let mut m = Self::Output::new();
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    unsafe {
                        let v = self.get_unchecked(i, k) * rhs.get_unchecked(k, j);
                        *m.get_unchecked_mut(i, j) += v;
                    }
                }
            }
        }
        m
    }
}


impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f32; N];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

