use super::MatrixNew;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign, Index, IndexMut};

impl_basic_ops!(Add, add, +, MatrixNew);
impl_basic_ops!(Sub, sub, -, MatrixNew);
impl_assign_ops!(AddAssign, add_assign, +=, MatrixNew);
impl_assign_ops!(SubAssign, sub_assign, -=, MatrixNew);
impl_scalar_ops!(Mul, mul, *, MatrixNew);
impl_scalar_ops!(Div, div, /, MatrixNew);

impl<const N: usize> Mul<MatrixNew<N>> for MatrixNew<N> {
    type Output = MatrixNew<N>;
    fn mul(self, rhs: MatrixNew<N>) -> Self::Output {
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

impl<'a, 'b, const N: usize> Mul<&'a MatrixNew<N>> for &'b MatrixNew<N>
where
    'a: 'b,
{
    type Output = MatrixNew<N>;
    fn mul(self, rhs: &'a MatrixNew<N>) -> Self::Output {
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


impl<const N: usize> Index<usize> for MatrixNew<N> {
    type Output = [f32; N];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for MatrixNew<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

