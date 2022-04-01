use super::FragmentShader;
use crate::algebra::vector_new::{vector3, Vector3};
use crate::pipeline::model::TriangulatedModel;
use crate::pipeline::{camera::Camera, light::Light, model::Triangle};
use crate::Color;

pub struct ColorShader;

const DEFAULT_COLOR: Vector3 = vector3([255.0, 255.0, 255.0]);

impl FragmentShader for ColorShader {
    fn shade(&self, model: &TriangulatedModel, _: &Triangle, _: (f32, f32, f32), _: f32) -> Color {
        model
            .material
            .as_ref()
            .map_or(Color::from(&DEFAULT_COLOR), |m| {
                Color::from(&m.diffuse_color)
            })

        /*
        if triangle.vertexs.iter().any(|v| v.color.is_none()) {
            Color::rgb(255, 255, 255)
        } else {
            let c0 = triangle.vertexs[0].color.as_ref().unwrap();
            let c1 = triangle.vertexs[1].color.as_ref().unwrap();
            let c2 = triangle.vertexs[2].color.as_ref().unwrap();
            c0 * alpha + c1 * beta + c2 * gamma
        }
        */
    }
    fn update_camera(&mut self, _: &Camera) {}
    fn update_light(&mut self, _: &Light) {}
}
