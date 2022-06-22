use crate::algebra::vector_new::vector3;

#[test]
fn load() {
    // println!("{:?}", vector3([32::NEG_ININITY, 32::NEG_ININITY, 32::NEG_ININITY]).norm() > 1.0)
    let c = vector3([0.747 + 0.058, 0.747 + 0.258, 0.747]) * 8.0
        + vector3([0.740 + 0.287, 0.740 + 0.160, 0.740]) * 15.6
        + vector3([0.737 + 0.642, 0.737 + 0.159, 0.737]) * 18.4;
    println!("{:?}", &c / c.x());
}
