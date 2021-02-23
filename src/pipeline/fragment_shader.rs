use super::model::Triangle;
use crate::Color;
pub mod color_shader;
pub mod texture_shader;
pub mod z_shader;

pub trait FragmentShader {
    fn shade(&self, triangle: &Triangle, barycenter: (f32, f32, f32), z: f32) -> Color;
}

pub use color_shader::ColorShader;
pub use texture_shader::TextureShader;
pub use z_shader::ZShader;

pub fn make_shader(name: &str, path: &str) -> Option<Box<dyn FragmentShader>> {
    match name.to_lowercase().as_ref() {
        "z" => Some(Box::new(ZShader {})),
        "color" => Some(Box::new(ColorShader {})),
        "texture" => Some(Box::new(TextureShader::new(path))),
        _ => panic!("Unknown shader: {}", name),
    }
}
