use super::rasterizer::FragmentBuffer;
use crate::shaders_builder;

pub mod depth_shader;
pub mod vertex_shader;

use depth_shader::DepthShader;
use vertex_shader::VertexShader;

pub trait Shader {
    //From fragments to framebuffer
    fn shade(fragments: &FragmentBuffer) -> Vec<Color>;
}

pub fn all_shaders() -> Vec<fn(&FragmentBuffer) -> Vec<Color>> {
    shaders_builder!(DepthShader, VertexShader)
}

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[macro_export]
macro_rules! shaders_builder {
    ($($shader: tt), *) => {
        {
            let v: Vec<fn(&FragmentBuffer) -> Vec<Color>> = vec![
                $(
                    $shader::shade,
                )*
            ];
            v
        }

    };
}

#[macro_export]
macro_rules! blend_color {
    ( $($color: expr), * ) => {
        {
            let mut r: f32 = 0.0;
            let mut g: f32 = 0.0;
            let mut b: f32 = 0.0;
            $(
                let (c, perc) = $color;
                r += c.r as f32 * perc ;
                g += c.g as f32 * perc;
                b += c.b as f32 * perc;
            )*
            let (r, g, b) = (r as u8, g as u8, b as u8);
            Color::rgba(r, g, b, 100u8)
        }
    };
}
