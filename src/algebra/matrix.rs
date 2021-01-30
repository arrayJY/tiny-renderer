mod ops;
use generic_array::typenum::{Prod, Unsigned};
use generic_array::typenum::{U1, U2, U3, U4};
use generic_array::{ArrayLength, GenericArray};
use std::iter::Sum;
use std::ops::{Mul, Sub};
use std::usize;
use typenum::{IsEqual, IsLessOrEqual, True};

#[derive(Debug, PartialEq)]
pub struct Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
{
    data: GenericArray<T, Prod<Row, Col>>,
}

#[allow(dead_code)]
impl<T, Row, Col> Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
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
            Some((0..self.rows()).map(move |row| &self[row][col]))
        } else {
            None
        }
    }

    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows() {
            Some((0..self.cols()).map(move |col| &self[row][col]))
        } else {
            None
        }
    }

    // Return the transposed matrix.
    // It won't change self.
    pub fn transpose(&self) -> Matrix<T, Col, Row>
    where
        T: Copy,
        Col: Mul<Row>,
        Prod<Col, Row>: ArrayLength<T>,
    {
        let cols = self.cols();
        let data = (0..cols)
            .map(move |col| self.get_col(col).unwrap())
            .flat_map(|v| v);
        let mut m = Matrix::<T, Col, Row>::new();

        let rows = m.rows();
        let cols = m.cols();
        let iter = (0..rows).flat_map(move |a| (0..cols).map(move |b| (a, b)));
        for ((row, col), value) in iter.zip(data) {
            m[row][col] = *value;
        }
        m
    }
}

impl<T, Row, Col> Matrix<T, Row, Col>
where
    T: Default + Copy + Mul<Output = T> + Sum,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
{
    pub fn dot(&self, rhs: &Matrix<T, Row, Col>) -> T {
        let r: T = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a * b)
            .sum();
        r
    }

    //only vector3 has cross product meaning.
    pub fn cross(&self, rhs: &Self) -> Self
    where
        T: Sub<Output = T>,
        Row: IsEqual<U3, Output = True>,
        Col: IsEqual<U1, Output = True>,
    {
        let mut m = Self::new();
        m[0][0] = self[1][0] * rhs[2][0] - self[2][0] * rhs[1][0];
        m[1][0] = self[2][0] * rhs[0][0] - self[0][0] * rhs[2][0];
        m[2][0] = self[0][0] * rhs[1][0] - self[1][0] * rhs[0][0];
        m
    }

    pub fn from<R, C>(matrix: &Matrix<T, R, C>) -> Self
    where
        T: Copy,
        R: Unsigned + Mul<C> + IsLessOrEqual<Row, Output = True>,
        C: Unsigned + IsLessOrEqual<Col, Output = True>,
        Prod<R, C>: ArrayLength<T>,
    {
        let mut m = Self::new();
        for (row, col) in matrix.index_iter() {
            m[row][col] = matrix[row][col];
        }
        m
    }

    fn index_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let rows = self.rows();
        let cols = self.cols();
        (0..rows).flat_map(move |a| (0..cols).map(move |b| (a, b)))
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

macro_rules! def_float_matrix {
    ($float: ty) => {
        #[allow(dead_code)]
        pub type Matrixf<Row, Col> = Matrix<$float, Row, Col>;
        #[allow(dead_code)]
        pub type Matrix1f = Matrix1<$float>;
        #[allow(dead_code)]
        pub type Matrix2f = Matrix2<$float>;
        #[allow(dead_code)]
        pub type Matrix3f = Matrix3<$float>;
        #[allow(dead_code)]
        pub type Matrix4f = Matrix4<$float>;
    };
}

def_float_matrix!(f32);

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

#[macro_export]
macro_rules! matrixf {
    ($dimension: ty; $($val: expr), *) => {
        {
            let mut m = Matrixf::<$dimension, $dimension>::new();
            let d = m.rows();
            let mut iter = (0..d).flat_map(move |a| (0..d).map(move |b| (a, b)));
            $(
                {
                    let (row, col) = iter.next().unwrap();
                    m[row][col] = $val;
                }
            )*
            m
       }
    };

    ($row: ty, $col: ty; $($val: expr), *) => {
        {
            let mut m = Matrixf::<$row, $col>::new();
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

#[allow(dead_code)]
impl Matrix4f {
    pub fn translation_matrix(x: f32, y: f32, z: f32) -> Matrix4f {
        matrixf!(U4;
            1.0, 0.0, 0.0,   x,
            0.0, 1.0, 0.0,   y,
            0.0, 0.0, 1.0,   z,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn scale_matrix(sx: f32, sy: f32, sz: f32) -> Matrix4f {
        matrixf!(U4;
             sx, 0.0, 0.0, 0.0,
            0.0,  sy, 0.0, 0.0,
            0.0, 0.0,  sz, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn rotate_around_x_matrix(angle: f32) -> Matrix4f {
        let sina = angle.sin();
        let cosa = angle.cos();
        matrixf!(U4;
            1.0,  0.0,   0.0, 0.0,
            0.0, cosa, -sina, 0.0,
            0.0, sina,  cosa, 0.0,
            0.0,  0.0,   0.0, 1.0
        )
    }

    pub fn rotate_around_y_matrix(angle: f32) -> Matrix4f {
        let sina = angle.sin();
        let cosa = angle.cos();
        matrixf!(U4;
             cosa, 0.0, sina, 0.0,
              0.0, 1.0,  0.0, 0.0,
            -sina, 0.0, cosa, 0.0,
              0.0, 0.0,  0.0, 1.0
        )
    }

    pub fn rotate_around_z_matrix(angle: f32) -> Matrix4f {
        let sina = angle.sin();
        let cosa = angle.cos();
        matrixf!(U4;
             cosa, sina, 0.0, 0.0,
             sina, cosa, 0.0, 0.0,
              0.0,  0.0, 1.0, 0.0,
              0.0,  0.0, 0.0, 1.0
        )
    }
}
