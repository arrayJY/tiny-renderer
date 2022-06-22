use super::FragmentShader;
use crate::algebra::vector_new::{vector3, Vector3};
use crate::pipeline::material::{MaterialNew};
use crate::pipeline::model::TriangulatedModel;
use crate::pipeline::{camera::Camera, light::Light, model::Triangle};
use crate::Color;

pub struct ColorShader;

const DEFAULT_COLOR: Vector3 = vector3([127.0, 127.0, 127.0]);

impl FragmentShader for ColorShader {
    fn shade(&self, model: &TriangulatedModel, _: &Triangle, _: (f32, f32, f32), _: f32) -> Color {
        model
            .material
            .as_ref()
            .map_or(Color::from(&DEFAULT_COLOR), |m| match m.as_ref() {
                MaterialNew::Phong(m) => Color::from(&m.diffuse_color),
                _ => Color::from(&DEFAULT_COLOR),
            })
    }
    fn update_camera(&mut self, _: &Camera) {}
    fn update_light(&mut self, _: &Light) {}
}
