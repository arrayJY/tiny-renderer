use super::Matrix;
use generic_array::typenum::Unsigned;
use generic_array::ArrayLength;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

macro_rules! impl_basic_ops {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, Row, Col> $trait for Matrix<T, Row, Col>
        where
            T: Default + Copy + $trait<Output = T>,
            Row: Unsigned + Mul<Col>,
            Col: Unsigned,
            <Row as Mul<Col>>::Output: ArrayLength<T>
        {
            type Output = Matrix<T, Row, Col>;
            fn $func(self, rhs: Matrix<T, Row, Col>) -> Self::Output {
                let mut m = Matrix::<T, Row, Col>::new();
                for row in 0..self.rows() {
                    for col in 0..self.cols() {
                        m[row][col] = self[row][col] $op rhs[row][col];
                    }
                }
                m
            }
        }

        impl<'a, 'b, T, Row, Col> $trait<&'a Matrix<T, Row, Col>> for &'b Matrix<T, Row, Col>
        where
            'a: 'b,
            T: Default + Copy + $trait<Output = T>,
            Row: Unsigned + Mul<Col>,
            Col: Unsigned,
            <Row as Mul<Col>>::Output: ArrayLength<T>
        {
            type Output = Matrix<T, Row, Col>;
            fn $func(self, rhs: &'a Matrix<T, Row, Col>) -> Self::Output {
                let mut m = Matrix::<T, Row, Col>::new();
                for row in 0..self.rows() {
                    for col in 0..self.cols() {
                        m[row][col] = self[row][col] $op rhs[row][col];
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
            Row: Unsigned + Mul<Col>,
            Col: Unsigned,
            <Row as Mul<Col>>::Output: ArrayLength<T>
        {
            fn $func(&mut self, rhs: Matrix<T, Row, Col>) {
                assert!((self.rows(), self.cols()) == (rhs.rows(), rhs.cols()));
                let rows = self.rows();
                let cols = self.cols();
                (0..rows)
                    .flat_map(move |a| (0..cols).map(move |b| (a, b)))
                    .for_each(|(row, col)| self[row][col] $op rhs[row][col]);
            }
        }
    };
}

impl_basic_assign_ops!(AddAssign, add_assign, +=);
impl_basic_assign_ops!(SubAssign, sub_assign, -=);

impl<T, Row, Col> Index<usize> for Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    <Row as Mul<Col>>::Output: ArrayLength<T>,
{
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let cols = self.cols();
        &self.data[index * cols..index * cols + cols]
    }
}

impl<T, Row, Col> IndexMut<usize> for Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    <Row as Mul<Col>>::Output: ArrayLength<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut [T]{
        let cols = self.cols();
        &mut self.data[index * cols..index * cols + cols]
    }
}
