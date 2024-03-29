use std::ops::{Mul, Neg};

use generic_array::ArrayLength;
use typenum::{Prod, Unsigned, U1, U2, U3, U4};

use crate::{input_matrix, vector4f};

use super::matrix::Matrix;

#[allow(dead_code)]
pub type Vector<T, N> = Matrix<T, N, U1>;

type VectorFloat = f32; //

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
pub type Vectorf<N> = Vector<VectorFloat, N>;
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

impl<T, N> Vector<T, N>
where
    T: Default,
    N: Unsigned + Mul<U1>,
    Prod<N, U1>: ArrayLength<T>,
{
    pub fn dimension(&self) -> usize {
        <N as Unsigned>::to_usize()
    }
}

impl<T: Default + Copy> Vector3<T> {
    pub fn x(&self) -> T {
        unsafe { *self.get_unchecked(0, 0) }
    }
    pub fn y(&self) -> T {
        unsafe { *self.get_unchecked(1, 0) }
    }
    pub fn z(&self) -> T {
        unsafe { *self.get_unchecked(2, 0) }
    }
}

impl<T: Default + Copy> Vector4<T> {
    pub fn x(&self) -> T {
        unsafe { *self.get_unchecked(0, 0) }
    }
    pub fn y(&self) -> T {
        unsafe { *self.get_unchecked(1, 0) }
    }
    pub fn z(&self) -> T {
        unsafe { *self.get_unchecked(2, 0) }
    }
    pub fn w(&self) -> T {
        unsafe { *self.get_unchecked(3, 0) }
    }

    pub fn new_point() -> Vector4f {
        vector4f!(0.0, 0.0, 0.0, 1.0)
    }

    pub fn new_vector() -> Vector4f {
        vector4f!(0.0, 0.0, 0.0, 0.0)
    }
}

impl Vector3f {
    pub fn from_vec4f(v: &Vector4f) -> Self {
        let mut r = Vector3f::new();
        unsafe {
            r.set_unchecked(0, 0, v.x());
            r.set_unchecked(1, 0, v.y());
            r.set_unchecked(2, 0, v.z());
        }
        r
    }
}

impl Vector4f {
    pub fn from_vec3f_point(v: &Vector3f) -> Vector4f {
        let mut r = Self::new();
        unsafe {
            r.set_unchecked(0, 0, v.x());
            r.set_unchecked(1, 0, v.y());
            r.set_unchecked(2, 0, v.z());
            r.set_unchecked(3, 0, 1.0);
        }
        r
    }
    pub fn from_vec3f_vector(v: &Vector3f) -> Vector4f {
        let mut r = Self::new();
        unsafe {
            r.set_unchecked(0, 0, v.x());
            r.set_unchecked(1, 0, v.y());
            r.set_unchecked(2, 0, v.z());
            r.set_unchecked(3, 0, 0.0);
        }
        r
    }
}

impl<N> Vectorf<N>
where
    N: Unsigned + Mul<U1>,
    Prod<N, U1>: ArrayLength<VectorFloat>,
{
    pub fn norm(&self) -> VectorFloat {
        self.into_iter().map(|v| v * v).sum::<VectorFloat>().sqrt()
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        self.into_iter().for_each(|v| {
            *v = *v / norm;
        })
    }

    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
}

impl<T, N> Neg for Vector<T, N>
where
    T: Default + Neg<Output = T> + Copy,
    N: Unsigned + Mul<U1>,
    Prod<N, U1>: ArrayLength<T>,
{
    type Output = Vector<T, N>;
    fn neg(self) -> Self::Output {
        let mut v = Self::Output::new();
        for (row, col) in self.index_iter() {
            v[row][col] = -self[row][col];
        }
        v
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
        Vector3f::new();
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
