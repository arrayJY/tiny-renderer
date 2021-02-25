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
