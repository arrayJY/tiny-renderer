use super::Vector;
use crate::algebra::matrix_new::Matrix;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign};

impl_basic_ops!(Add, add, +, Vector);
impl_basic_ops!(Sub, sub, -, Vector);
impl_assign_ops!(AddAssign, add_assign, +=, Vector);
impl_assign_ops!(SubAssign, sub_assign, -=, Vector);
impl_scalar_ops!(Mul, mul, *, Vector);
impl_scalar_ops!(Div, div, /, Vector);

impl<const N: usize> Mul<Vector<N>> for Matrix<N> {
    type Output = Vector<N>;
    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut m = Self::Output::new();
        for i in 0..N {
            unsafe {
                for j in 0..N {
                    let r1 = *rhs.0.get_unchecked(j);
                    let l = m.0.get_unchecked_mut(i);
                    *l += r1 * self.get_unchecked(i, j);
                }
            }
        }
        m
    }
}

impl<'a, 'b, const N: usize> Mul<&'a Vector<N>> for &'b Matrix<N>
where
    'a: 'b,
{
    type Output = Vector<N>;
    fn mul(self, rhs: &'a Vector<N>) -> Self::Output {
        let mut m = Self::Output::new();
        for i in 0..N {
            unsafe {
                for j in 0..N {
                    let r1 = *rhs.0.get_unchecked(j);
                    let l = m.0.get_unchecked_mut(i);
                    *l += r1 * self.get_unchecked(i, j);
                }
            }
        }
        m
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Vector<N>;
    fn neg(self) -> Self::Output {
        let mut m = Self::Output::new();
        m.data_iter_mut()
            .zip(self.data_iter())
            .for_each(|(l, r)| *l = -*r);
        m
    }
}
