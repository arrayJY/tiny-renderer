use super::matrix::Matrix;
use typenum::{U1, U2, U3, U4};

#[allow(dead_code)]
pub type Vector<T, N> = Matrix<T, N, U1>;

macro_rules! def_dimension_vector {
    ($name: ident, $dimension: ty) => {
        #[allow(dead_code)]
        pub type $name<T> = Vector<T, $dimension>;
    };
}

def_dimension_vector!(Vector1, U1);
def_dimension_vector!(Vector2, U2);
def_dimension_vector!(Vector3, U3);
def_dimension_vector!(Vector4, U4);

macro_rules! def_float_dimension_vector {
    ($name: ident, $dimension: ty) => {
        #[allow(dead_code)]
        pub type $name = Vector<f64, $dimension>;
    };
}
def_float_dimension_vector!(Vector1f, U1);
def_float_dimension_vector!(Vector2f, U2);
def_float_dimension_vector!(Vector3f, U3);
def_float_dimension_vector!(Vector4f, U4);
