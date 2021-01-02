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
    ($type: ty; $length: ty) => (
        Matrix::<$type, $length, $length>::new();
    );
    ($type: ty; $row: ty, $col: ty ) => (
        Matrix::<$type, $row, $col>::new();
    );
}