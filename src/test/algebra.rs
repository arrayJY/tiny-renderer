#[allow(unused_imports)]
use crate::algebra::typenum::{U1,U2,U3};
#[allow(unused_imports)]
use crate::{algebra::matrix::Matrix, matrix};
#[test]
fn matrix_index() {
    let m = matrix!(i32; U2; 1, 2, 3, 4);
    assert_eq!(m[0][0], 1);
    assert_eq!(m[1][1], 4);
}

#[test]
#[should_panic]
fn matrix_index_out_of_range() {
    let m = matrix!(i32; U2; 1, 2, 3, 4);
    m[1][2];
}

#[test]
fn matrix_add() {
    let m1 = matrix!(i32; U2; 1, 2, 3, 4);
    let m2 = matrix!(i32; U2; 4, 3, 2, 1);
    assert_eq!(matrix!(i32;U2;5,5,5,5), m1 + m2);
}

#[test]
fn matrix_sub() {
    let m1 = matrix!(i32; U2; 1, 2, 3, 4);
    let m2 = matrix!(i32; U2; 4, 3, 2, 1);
    assert_eq!(matrix!(i32;U2;-3,-1,1,3), m1 - m2);
}

#[test]
fn matrix_mul() {
    let m1 = matrix!(i32; U2; 1, 2, 3, 4);
    let m2 = matrix!(i32; U2; 4, 3, 2, 1);
    assert_eq!(matrix!(i32;U2;8,5,20,13), m1 * m2);
}

#[test]
fn matrix_dot() {
    let m1 = matrix!(i32; U2; 1, 2, 3, 4);
    let m2 = matrix!(i32; U2; 4, 3, 2, 1);
    assert_eq!(20, m1.dot(&m2));
}

#[test]
fn matrix_cross() {
    let m1 = matrix!(i32; U3, U1; 1, 2, 3);
    let m2 = matrix!(i32; U3, U1; 0, 1, 2);
    assert_eq!(matrix!(i32; U3, U1; 1, -2, 1) ,  m1.cross(&m2));
}

#[test]
fn matrix_transpose() {
    let m1 = matrix!(i32; U3, U1; 1, 2, 3);
    assert_eq!(matrix!(i32; U1, U3; 1, 2, 3), m1.transpose());
}


