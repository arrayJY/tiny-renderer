use super::rasterizer::FragmentBuffer;

pub mod depth_shader;

pub trait Shader {
    //From fragments to framebuffer
    fn shade(fragments: &FragmentBuffer) -> Vec<Color>;
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
