use crate::algebra::matrix::Matrixf;
use generic_array::ArrayLength;
use std::{fmt::Debug, ops::Mul};
use typenum::{Prod, Unsigned};

mod algebra;
mod algebra_new;
mod pbr;

#[allow(dead_code)]
pub fn about_equal<Row, Col>(m1: &Matrixf<Row, Col>, m2: &Matrixf<Row, Col>) -> Result<(), String>
where
    Row: Unsigned + Mul<Col> + Debug,
    Col: Unsigned + Debug,
    Prod<Row, Col>: ArrayLength<f32>,
{
    for (i, j) in m1.index_iter() {
        let v1 = m1[i][j];
        let v2 = m2[i][j];
        if (v1 - v2).abs() >= 1e-5 {
            return Err(format!("{:?} !≈ {:?}", m1, m2));
        }
    }
    Ok(())
}
