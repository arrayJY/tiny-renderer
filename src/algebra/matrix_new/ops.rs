use super::MatrixNew;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign, Div};

macro_rules! impl_scalar_ops {
    ($trait: ident, $func: ident, $op: tt, $type: ident) => {
        impl<const N: usize> $trait<f32> for $type<N> {
            type Output = $type<N>;
            fn $func(self, rhs: f32) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut().for_each(|v| *v = *v $op rhs);
                m
            }
        }

        impl<const N: usize> $trait<f32> for &$type<N> {
            type Output = $type<N>;
            fn $func(self, rhs: f32) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut().for_each(|v| *v = *v $op rhs);
                m
            }
        }
    };
}


/*
impl<const N: usize, M: Borrow<MatrixNew<N>>> Mul<f32> for M {

}
*/

macro_rules! impl_basic_ops {
    ($trait: ident, $func: ident, $op: tt, $type: ident) => {
        impl<const N: usize> $trait<$type<N>> for $type<N> {
            type Output = $type<N>;
            fn $func(self, rhs: $type<N>) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut()
                    .zip(self.data_iter().zip(rhs.data_iter()))
                    .for_each(|(l, (r1, r2))| *l = r1 $op r2);
                m
            }
        }

        impl<'a, 'b, const N: usize> $trait<&'a $type<N>> for &'b $type<N>
        where
            'a: 'b,
        {
            type Output = $type<N>;
            fn $func(self, rhs: &'a $type<N>) -> Self::Output {
                let mut m = Self::Output::new();
                m.data_iter_mut()
                    .zip(self.data_iter().zip(rhs.data_iter()))
                    .for_each(|(l, (r1, r2))| *l = r1 $op r2);
                m
            }
        }
    };
}

macro_rules! impl_assign_ops {
    ($trait: ident, $func: ident, $op: tt, $type: ident) => {
        impl<const N: usize> $trait for $type<N> {
            fn $func(&mut self, rhs: Self) {
                self.data_iter_mut()
                    .zip(rhs.data_iter())
                    .for_each(|(l, r)| {
                        *l $op r;
                    })
            }
        }

        impl<'a, 'b, const N: usize> $trait<&'a $type<N>> for &'b mut $type<N>
        where
            'a: 'b,
        {
            fn $func(&mut self, rhs: &'a $type<N>) {
                self.data_iter_mut()
                    .zip(rhs.data_iter())
                    .for_each(|(l, r)| {
                        *l $op r;
                    });
            }
        }
    };
}

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
