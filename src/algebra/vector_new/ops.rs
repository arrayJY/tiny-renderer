use super::VectorNew;
use crate::algebra::matrix_new::MatrixNew;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign};

impl_basic_ops!(Add, add, +, VectorNew);
impl_basic_ops!(Sub, sub, -, VectorNew);
impl_assign_ops!(AddAssign, add_assign, +=, VectorNew);
impl_assign_ops!(SubAssign, sub_assign, -=, VectorNew);
impl_scalar_ops!(Mul, mul, *, VectorNew);
impl_scalar_ops!(Div, div, /, VectorNew);

impl<const N: usize> Mul<VectorNew<N>> for MatrixNew<N> {
    type Output = VectorNew<N>;
    fn mul(self, rhs: VectorNew<N>) -> Self::Output {
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

impl<'a, 'b, const N: usize> Mul<&'a VectorNew<N>> for &'b MatrixNew<N>
where
    'a: 'b,
{
    type Output = VectorNew<N>;
    fn mul(self, rhs: &'a VectorNew<N>) -> Self::Output {
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

impl<const N: usize> Index<usize> for VectorNew<N> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for VectorNew<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Neg for VectorNew<N> {
    type Output = VectorNew<N>;
    fn neg(self) -> Self::Output {
        let mut m = Self::Output::new();
        m.data_iter_mut()
            .zip(self.data_iter())
            .for_each(|(l, r)| *l = -*r);
        m
    }
}
