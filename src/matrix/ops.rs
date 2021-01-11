use super::Matrix;
use generic_array::{ArrayLength, GenericArray};
use std::ops::{Add, AddAssign, Index, Sub, SubAssign};

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
                            self[row][col] $op rhs[row][col]
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
                            self[row][col] $op rhs[row][col]
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

macro_rules! impl_basic_assign_ops {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, Row, Col> $trait for Matrix<T, Row, Col>
        where
            T: Default + Copy + $trait,
            Row: ArrayLength<GenericArray<T, Col>>,
            Col: ArrayLength<T>,
        {
            fn $func(&mut self, rhs: Matrix<T, Row, Col>) {
                assert!((self.rows(), self.cols()) == (rhs.rows(), rhs.cols()));
                let rows = self.rows();
                let cols = self.cols();
                (0..rows)
                    .flat_map(move |a| (0..cols).map(move |b| (a, b)))
                    .for_each(|(row, col)| self.data[row][col] $op rhs.data[row][col]);
            }
        }
    };
}

impl_basic_assign_ops!(AddAssign, add_assign, +=);
impl_basic_assign_ops!(SubAssign, sub_assign, -=);

impl<T, Row, Col> Index<usize> for Matrix<T, Row, Col>
where
    T: Default + Copy,
    Row: ArrayLength<GenericArray<T, Col>>,
    Col: ArrayLength<T>,
{
    type Output = GenericArray<T, Col>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
