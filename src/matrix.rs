pub mod ops;
use generic_array::typenum::{Prod, Unsigned};
use generic_array::typenum::{U1, U2, U3, U4};
use generic_array::{ArrayLength, GenericArray};
use std::ops::Mul;
use std::usize;

#[derive(Debug)]
pub struct Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    <Row as Mul<Col>>::Output: ArrayLength<T>,
{
    data: GenericArray<T, Prod<Row, Col>>,
}

#[allow(dead_code)]
impl<T, Row, Col> Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    <Row as Mul<Col>>::Output: ArrayLength<T>,
{
    pub fn new() -> Matrix<T, Row, Col> {
        Matrix::<T, Row, Col> {
            data: GenericArray::default(),
        }
    }

    pub fn rows(&self) -> usize {
        <Row as Unsigned>::to_u64() as usize
    }

    pub fn cols(&self) -> usize {
        <Col as Unsigned>::to_u64() as usize
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows() && col < self.cols() {
            Some(&self[row][col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows() && col < self.cols() {
            Some(&self[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), ()> {
        if row < self.rows() && col < self.cols() {
            self[row][col] = value;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.cols() {
            Some((0..self.rows()).map(move |row| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows() {
            Some((0..self.cols()).map(move |col| self.get(row, col).unwrap()))
        } else {
            None
        }
    }
}

#[allow(unused_macros)]
macro_rules! def_square_matrix {
    ($name: ident, $length : ty) => {
        #[allow(dead_code)]
        pub type $name<T> = Matrix<T, $length, $length>;
    };
}

def_square_matrix!(Matrix1, U1);
def_square_matrix!(Matrix2, U2);
def_square_matrix!(Matrix3, U3);
def_square_matrix!(Matrix4, U4);

#[macro_export]
macro_rules! matrix {
    ($type: ty; $length: ty) => {
        Matrix::<$type, $length, $length>::new();
    };
    ($type: ty; $row: ty, $col: ty ) => {
        Matrix::<$type, $row, $col>::new();
    };
    ($type: ty; $length: ty; $($val: expr), * ) => {
        {
            let mut m = Matrix::<$type, $length, $length>::new();
            let length = m.rows();
            let mut iter = (0..length).flat_map(move |a| (0..length).map(move |b| (a, b)));
            $(
                {
                    let (row, col) = iter.next().unwrap();
                    m[row][col] = $val;
                }
            )*
            m
        }
    };
    ($type: ty; $row: ty, $col: ty; $($val: expr), * ) => {
        {
            let mut m = Matrix::<$type, $row, $col>::new();
            let rows = m.rows();
            let cols = m.cols();
            let mut iter = (0..rows).flat_map(move |a| (0..cols).map(move |b| (a, b)));
            $(
                {
                    let (row, col) = iter.next().unwrap();
                    m[row][col] = $val;
                }
            )*
            m
        }
    };
}
