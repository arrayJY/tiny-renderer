use crate::algebra::vector_new::Vector4;

pub struct AreaLight {
    pub r: f32,
    pub point: Vector4,  //center
    pub normal: Vector4, //normal
}
