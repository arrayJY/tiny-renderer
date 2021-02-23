use super::model::Triangle;
use crate::shaders_builder;
use crate::Color;
pub mod color_shader;
pub mod z_shader;

pub type ShaderFunc = Box<dyn Fn(&Triangle, (f32, f32, f32), f32) -> Color>;
pub trait FragmentShader {
    fn shader() -> ShaderFunc;
}

pub use color_shader::ColorShader;
pub use z_shader::ZShader;

pub fn all_shaders() -> Vec<fn() -> ShaderFunc> {
    shaders_builder![ColorShader, ZShader]
}

#[macro_export]
macro_rules! shaders_builder {
    ($($shader: tt), *) => {
        {
            let v: Vec<fn() -> ShaderFunc> = vec![
                $(
                    $shader::shader,
                )*
            ];
            v
        }

    };
}
