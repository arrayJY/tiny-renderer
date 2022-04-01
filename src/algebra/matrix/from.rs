use std::ops::Mul;

use generic_array::ArrayLength;
use typenum::{Prod, Unsigned};

use super::{Matrix, Matrix3f, Matrix4f};
impl<T, Row, Col> From<&Matrix<T, Row, Col>> for Matrix<T, Row, Col>
where
    T: Default + Clone,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
{
    fn from(other: &Matrix<T, Row, Col>) -> Matrix<T, Row, Col> {
        Matrix {
            data: other.data.clone(),
        }
    }
}
impl From<&Matrix4f> for Matrix3f {
    fn from(matrix: &Matrix4f) -> Self {
        Matrix3f {
            data: matrix
                .data
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| {
                    if i / 4 != 3 && i % 4 != 3 {
                        Some(v)
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}
