use crate::algebra::vector::Vector4f;

pub struct AreaLight {
    pub r: f32,
    pub point: Vector4f,  //center
    pub normal: Vector4f  //normal 
}