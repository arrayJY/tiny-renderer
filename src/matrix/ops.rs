use super::Matrix;
use generic_array::{ArrayLength, GenericArray};
use std::ops::{Add, Sub};

macro_rules! impl_basic_ops {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, Row, Col> $trait for Matrix<T, Row, Col>
        where
            T: Default + Copy + $trait<Output = T>,
            Row: ArrayLength<GenericArray<T, Col>>,
            Col: ArrayLength<T>,
        {
            type Output = Matrix<T, Row, Col>;
            fn $func(self, rhs: Matrix<T, Row, Col>) -> Self::Output {
                let mut m = Matrix::<T, Row, Col>::new();
                for row in 0..self.rows() {
                    for col in 0..self.cols() {
                        m.set(
                            row,
                            col,
                            *self.get(row, col).unwrap() $op *rhs.get(row, col).unwrap()
                        )
                        .unwrap();
                    }
                }
                m
            }
        }

        impl<'a, 'b, T, Row, Col> $trait<&'a Matrix<T, Row, Col>> for &'b Matrix<T, Row, Col>
        where
            'a: 'b,
            T: Default + Copy + $trait<Output = T>,
            Row: ArrayLength<GenericArray<T, Col>>,
            Col: ArrayLength<T>,
        {
            type Output = Matrix<T, Row, Col>;
            fn $func(self, rhs: &'a Matrix<T, Row, Col>) -> Self::Output {
                let mut m = Matrix::<T, Row, Col>::new();
                for row in 0..self.rows() {
                    for col in 0..self.cols() {
                        m.set(
                            row,
                            col,
                            *self.get(row, col).unwrap() $op *rhs.get(row, col).unwrap()
                        )
                        .unwrap();
                    }
                }
                m
            }
        }
    };
}

impl_basic_ops!(Add, add, +);
impl_basic_ops!(Sub, sub, -);
