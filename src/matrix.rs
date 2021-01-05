use std::usize;

use generic_array::typenum::Unsigned;
use generic_array::typenum::{U1, U2, U3, U4};
use generic_array::{ArrayLength, GenericArray};

#[derive(Debug)]
pub struct Matrix<T, Row, Col>
where
    T: Default,
    Row: ArrayLength<GenericArray<T, Col>>,
    Col: ArrayLength<T>,
{
    data: GenericArray<GenericArray<T, Col>, Row>,
}

#[allow(dead_code)]
impl<T, Row, Col> Matrix<T, Row, Col>
where
    T: Default,
    Row: ArrayLength<GenericArray<T, Col>>,
    Col: ArrayLength<T>,
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
            Some(&self.data[row][col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows() && col < self.cols() {
            Some(&self.data[row][col])
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), ()> {
        if row < self.rows() && col < self.cols() {
            self.data[row][col] = value;
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
}
