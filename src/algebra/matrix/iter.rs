use super::Matrix;
use generic_array::{ArrayLength, GenericArrayIter};
use std::ops::Mul;
use typenum::{Prod, Unsigned};


impl<T, Row, Col> IntoIterator for Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
{
    type Item = T;
    type IntoIter = GenericArrayIter<T, Prod<Row, Col>>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T, Row, Col> IntoIterator for &'a Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
{
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T, Row, Col> IntoIterator for &'a mut Matrix<T, Row, Col>
where
    T: Default,
    Row: Unsigned + Mul<Col>,
    Col: Unsigned,
    Prod<Row, Col>: ArrayLength<T>,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
