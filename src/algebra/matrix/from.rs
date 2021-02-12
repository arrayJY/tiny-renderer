use std::ops::Mul;

use generic_array::ArrayLength;
use typenum::{Prod, Unsigned};

use super::Matrix;
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
