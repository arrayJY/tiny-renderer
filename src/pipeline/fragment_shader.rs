use super::model::Triangle;
use crate::Color;
pub mod color_shader;
pub mod z_shader;

pub type ShaderFunc = Box<dyn Fn(&Triangle, (f32, f32, f32), f32) -> Color>;
pub trait FragmentShader {
    fn shader(&self) -> ShaderFunc;
}

pub use color_shader::ColorShader;
pub use z_shader::ZShader;

pub fn make_shader(name: &str, path: &str) -> Option<Box<dyn FragmentShader>> {
    match name.to_lowercase().as_ref() {
        "z" => Some(Box::new(ZShader {})),
        "color" => Some(Box::new(ColorShader {})),
        _ => panic!("Unknown shader: {}", name),
    }
}