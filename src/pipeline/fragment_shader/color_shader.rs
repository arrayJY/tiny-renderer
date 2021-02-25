use super::FragmentShader;
use crate::pipeline::{camera::Camera, light::Light, model::Triangle};
use crate::Color;

pub struct ColorShader;

impl FragmentShader for ColorShader {
    fn shade(&self, triangle: &Triangle, (alpha, beta, gamma): (f32, f32, f32), _: f32) -> Color {
        if triangle.vertexs.iter().any(|v| v.color.is_none()) {
            Color::rgb(255, 255, 255)
        } else {
            let c0 = triangle.vertexs[0].color.as_ref().unwrap();
            let c1 = triangle.vertexs[1].color.as_ref().unwrap();
            let c2 = triangle.vertexs[2].color.as_ref().unwrap();
            c0 * alpha + c1 * beta + c2 * gamma
        }
    }
    fn update_camera(&mut self, _: &Camera) {}
    fn update_light(&mut self, _: &Light) {}
}
