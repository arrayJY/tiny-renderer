use super::matrix::Matrix;
use typenum::{U1, U2, U3, U4};
use crate::{input_matrix, vector4f};

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

#[allow(dead_code)]
pub type Vectorf<N> = Vector<f32, N>;
#[allow(dead_code)]
pub type Vector1f = Vectorf<U1>;
#[allow(dead_code)]
pub type Vector2f = Vectorf<U2>;
#[allow(dead_code)]
pub type Vector3f = Vectorf<U3>;
#[allow(dead_code)]
pub type Vector4f = Vectorf<U4>;

#[macro_export]
macro_rules! vector {
    ($type: ty, $dimension: ty) => {
        Vector::<$type, $dimension>::new()
    };
    ($type: ty; $dimension : ty; $($val: expr), * ) => {
        {
        let mut v = Vector::<$type, $dimension>::new();
        let mut iter = (0..v.rows());
        $(
            {
                let i = iter.next().unwrap();
                v[i][0] = $val;
            }
        )*
        v
        }
    };
}

impl<T: Default + Copy> Vector3<T>
{
    pub fn x(&self) -> T {
        self[0][0]
    }
    pub fn y(&self) -> T {
        self[0][1]
    }
    pub fn z(&self) -> T {
        self[0][2]
    }
}

impl<T: Default + Copy> Vector4<T>
{
    pub fn x(&self) -> T {
        self[0][0]
    }
    pub fn y(&self) -> T {
        self[0][1]
    }
    pub fn z(&self) -> T {
        self[0][2]
    }
    pub fn w(&self) -> T {
        self[0][3]
    }

    pub fn new_point() -> Vector4f {
        vector4f!(0.0, 0.0, 0.0, 1.0)
    }

    pub fn new_vector() -> Vector4f {
        vector4f!(0.0, 0.0, 0.0, 0.0)
    }
}

#[macro_export]
macro_rules! vectorf {
    ($dimension: ty) => {
        Vectorf::<$dimension>::new()
    };
    ($dimension : ty; $($val: expr), * ) => {
        {
        let mut v = Vectorf::<$dimension>::new();
        let mut iter = 0..v.rows();
        $(
            {
                let i = iter.next().unwrap();
                v[i][0] = $val;
            }
        )*
        v
        }
    };
}

#[macro_export]
macro_rules! vector1f {
    () => {
        Vector1f::new();
    };
    ($($val: expr), +) => {
        {
            let mut v = Vector1f::new();
            input_matrix! (v, $($val), +);
            v
        }
    };
}

#[macro_export]
macro_rules! vector2f {
    () => {
        Vector2f::new();
    };
    ($($val: expr), +) => {
        {
            let mut v = Vector2f::new();
            input_matrix! (v, $($val), +);
            v
        }
    };
}

#[macro_export]
macro_rules! vector3f {
    () => {
        Vectorrf::new();
    };
    ($($val: expr), +) => {
        {
            let mut v = Vector3f::new();
            input_matrix! (v, $($val), +);
            v
        }
    };
}

#[macro_export]
macro_rules! vector4f {
    () => {
        Vector4f::new();
    };
    ($($val: expr), +) => {
        {
            let mut v = Vector4f::new();
            input_matrix! (v, $($val), +);
            v
        }
    };
}
