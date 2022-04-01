use super::FragmentShader;
use crate::{
    pipeline::{
        camera::Camera,
        light::Light,
        model::{Triangle, TriangulatedModel},
    },
    Color,
};

pub struct ZShader;

#[allow(unused_variables)]
impl FragmentShader for ZShader {
    fn shade(
        &self,
        _: &TriangulatedModel,
        triangle: &Triangle,
        barycenter: (f32, f32, f32),
        z: f32,
    ) -> Color {
        &Color::rgb(255, 255, 255) * (z * 5.0 - 4.0)
    }
    fn update_camera(&mut self, _: &Camera) {}
    fn update_light(&mut self, _: &Light) {}
}
