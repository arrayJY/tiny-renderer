use super::{camera::Camera, light::Light, model::Triangle};
use crate::{renderer::Renderer, Color};
pub mod color_shader;
pub mod phong_shader;
pub mod texture_shader;
pub mod z_shader;

pub trait FragmentShader {
    fn shade(&self, triangle: &Triangle, barycenter: (f32, f32, f32), z: f32) -> Color;
    fn update_camera(&mut self, camera: &Camera);
    fn update_light(&mut self, light: &Light);
}

pub use color_shader::ColorShader;
pub use phong_shader::PhongShader;
pub use texture_shader::TextureShader;
pub use z_shader::ZShader;

pub fn make_shader(name: &str, path: &str, renderer: &Renderer) -> Option<Box<dyn FragmentShader>> {
    match name.to_lowercase().as_ref() {
        "z" => Some(Box::new(ZShader {})),
        "color" => Some(Box::new(ColorShader {})),
        "texture" => Some(Box::new(TextureShader::new(path))),
        "phong-color" => Some(Box::new(PhongShader::color_shader(renderer))),
        "phong-texture" => Some(Box::new(PhongShader::texture_shader(renderer, path))),
        _ => None,
    }
}

#[macro_export]
macro_rules! interpolate {
    ($triangle: tt, $attr: ident; $barycenter: expr) => {{
        let (alpha, beta, gamma) = $barycenter;
        let v1 = $triangle.vertexs[0].$attr.as_ref().unwrap();
        let v2 = $triangle.vertexs[1].$attr.as_ref().unwrap();
        let v3 = $triangle.vertexs[2].$attr.as_ref().unwrap();
        v1 * alpha + v2 * beta + v3 * gamma
    }};
}

#[macro_export]
macro_rules! interpolate_uv {
    ($triangle: tt, $attr: ident; $barycenter: expr) => {{
        let (alpha, beta, gamma) = $barycenter;
        let &(u0, v0) = $triangle.vertexs[0].$attr.as_ref().unwrap();
        let &(u1, v1) = $triangle.vertexs[1].$attr.as_ref().unwrap();
        let &(u2, v2) = $triangle.vertexs[2].$attr.as_ref().unwrap();
        (
            u0 * alpha + u1 * beta + u2 * gamma,
            v0 * alpha + v1 * beta + v2 * gamma,
        )
    }};
}
