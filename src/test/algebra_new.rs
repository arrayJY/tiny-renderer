#[allow(unused_imports)]
#[allow(unused_imports)]
use crate::{
    algebra::{
        matrix_new::{Matrix, Matrix3, Matrix4},
        vector_new::{Vector, Vector3, Vector4},
    },
    *,
};
#[test]
fn matrix_index() {
    let m = Matrix::<2>([[1.0, 2.0], [3.0, 4.0]]);
    // let m = matrix!(i32; U2; 1, 2, 3, 4);
    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][1], 2.0);
    assert_eq!(m[1][0], 3.0);
    assert_eq!(m[1][1], 4.0);
}

#[test]
#[should_panic]
fn matrix_index_out_of_range() {
    let m = Matrix::<2>([[1.0, 2.0], [3.0, 4.0]]);
    m[1][2];
}

#[test]
fn matrix_add() {
    let m1 = Matrix::<2>([[1.0, 2.0], [3.0, 4.0]]);
    let m2 = Matrix::<2>([[4.0, 3.0], [2.0, 1.0]]);
    let m3 = Matrix::<2>([[5.0, 5.0], [5.0, 5.0]]);
    assert_eq!(m3, m1 + m2);
}

#[test]
fn matrix_sub() {
    let m1 = Matrix::<2>([[1.0, 2.0], [3.0, 4.0]]);
    let m2 = Matrix::<2>([[4.0, 3.0], [2.0, 1.0]]);
    let m3 = Matrix::<2>([[-3.0, -1.0], [1.0, 3.0]]);
    assert_eq!(m3, m1 - m2);
}

#[test]
fn matrix_mul() {
    let m1 = Matrix::<2>([[1.0, 2.0], [3.0, 4.0]]);
    let m2 = Matrix::<2>([[4.0, 3.0], [2.0, 1.0]]);
    let m3 = Matrix::<2>([[8.0, 5.0], [20.0, 13.0]]);
    assert_eq!(m3, m1 * m2);
}

#[test]
fn vector_dot() {
    let v1 = Vector::<4>([1.0, 2.0, 3.0, 4.0]);
    let v2 = Vector::<4>([4.0, 3.0, 2.0, 1.0]);
    assert_eq!(20.0, v1.dot(&v2));
}

#[test]
fn vector_cross() {
    let v1 = Vector::<3>([1.0, 2.0, 3.0]);
    let v2 = Vector::<3>([0.0, 1.0, 2.0]);
    let v3 = Vector::<3>([1.0, -2.0, 1.0]);
    assert_eq!(v3, v1.cross(&v2));
}

#[test]
fn matrix_transpose() {
    let m1 = Matrix::<2>([[1.0, 2.0], [3.0, 4.0]]);
    let m2 = Matrix::<2>([[1.0, 3.0], [2.0, 4.0]]);
    assert_eq!(m2, m1.transpose());
}

#[test]
fn matrix_unit() {
    let m1 = Matrix::<3>([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]);
    let m2 = Matrix::<3>::unit();
    assert_eq!(m2, m1);
}

#[test]
fn normalize_vector() {
    let mut v = Vector::<3>([0.0, 0.0, 2.0]);
    v.normalize();
    assert_eq!(v, Vector::<3>([0.0, 0.0, 1.0]))
}
