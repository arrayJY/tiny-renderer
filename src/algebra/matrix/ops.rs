use super::Matrix;
use generic_array::typenum::Unsigned;
use generic_array::ArrayLength;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};
use std::{iter::Sum, ops::Div};
use typenum::Prod;

macro_rules! impl_basic_ops {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, Row, Col> $trait for Matrix<T, Row, Col>
        where
            T: Default + Copy + $trait<Output = T>,
            Row: Unsigned + Mul<Col>,
            Col: Unsigned,
            Prod<Row, Col>: ArrayLength<T>,
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
            Prod<Row, Col>: ArrayLength<T>,
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
            Prod<Row, Col>: ArrayLength<T>,
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

impl<T, Row, Col, Col2> Mul<Matrix<T, Col, Col2>> for Matrix<T, Row, Col>
where
    T: Default + Copy + Mul<Output = T> + Sum<T>,
    Row: Unsigned + Mul<Col> + Mul<Col2>,
    Col: Unsigned + Mul<Col2>,
    Col2: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
    Prod<Row, Col2>: ArrayLength<T>,
    Prod<Col, Col2>: ArrayLength<T>,
{
    type Output = Matrix<T, Row, Col2>;
    fn mul(self, rhs: Matrix<T, Col, Col2>) -> Self::Output {
        let mut m = Matrix::<T, Row, Col2>::new();
        for row in 0..self.rows() {
            for col in 0..rhs.cols() {
                m[row][col] = self
                    .get_row(row)
                    .unwrap()
                    .zip(rhs.get_col(col).unwrap())
                    .map(|(a, b)| *a * *b)
                    .sum();
            }
        }
        m
    }
}

impl<'a, 'b, T, Row, Col, Col2> Mul<&'a Matrix<T, Col, Col2>> for &'b Matrix<T, Row, Col>
where
    'a: 'b,
    T: Default + Copy + Mul<Output = T> + Sum<T>,
    Row: Unsigned + Mul<Col> + Mul<Col2>,
    Col: Unsigned + Mul<Col2>,
    Col2: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
    Prod<Row, Col2>: ArrayLength<T>,
    Prod<Col, Col2>: ArrayLength<T>,
{
    type Output = Matrix<T, Row, Col2>;
    fn mul(self, rhs: &'a Matrix<T, Col, Col2>) -> Self::Output {
        let mut m = Matrix::<T, Row, Col2>::new();
        for row in 0..self.rows() {
            for col in 0..rhs.cols() {
                m[row][col] = self
                    .get_row(row)
                    .unwrap()
                    .zip(rhs.get_col(col).unwrap())
                    .map(|(a, b)| *a * *b)
                    .sum();
            }
        }
        m
    }
}

impl<T, Row, Col> Index<usize> for Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
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
    Prod<Row, Col>: ArrayLength<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        let cols = self.cols();
        &mut self.data[index * cols..index * cols + cols]
    }
}

macro_rules! impl_scalar_ops {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, Row, Col> $trait<T> for Matrix<T, Row, Col>
        where
            T: Default + Copy + $trait<Output = T>,
            Row: Unsigned + Mul<Col>,
            Col: Unsigned,
            Prod<Row, Col>: ArrayLength<T>,
        {
            type Output = Matrix<T, Row, Col>;
            fn $func(self, scalar: T) -> Self::Output {
                Matrix {
                    data: self.data.iter().map(|&v| v $op scalar).collect()
                }
            }
        }

        impl<T, Row, Col> $trait<T> for &Matrix<T, Row, Col>
        where
            T: Default + Copy + $trait<Output = T>,
            Row: Unsigned + Mul<Col>,
            Col: Unsigned,
            Prod<Row, Col>: ArrayLength<T>,
        {
            type Output = Matrix<T, Row, Col>;
            fn $func(self, scalar: T) -> Self::Output {
                Matrix {
                    data: self.data.iter().map(|&v| v $op scalar).collect()
                }
            }
        }
    };
}

impl_scalar_ops!(Mul, mul, *);
impl_scalar_ops!(Div, div, /);
