use super::FragmentShader;
use crate::{pipeline::model::Triangle, Color};

pub struct ZShader;

#[allow(unused_variables)]
impl FragmentShader for ZShader {
    fn shade(&self, triangle: &Triangle, barycenter: (f32, f32, f32), z: f32) -> Color {
        &Color::rgb(255, 255, 255) * z
    }
}
