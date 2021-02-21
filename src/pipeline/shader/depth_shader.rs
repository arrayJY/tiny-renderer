use crate::pipeline::rasterizer::FragmentBuffer;
use super::{Color, Shader};

#[allow(dead_code)]
pub struct DepthShader;

#[allow(dead_code)]
impl DepthShader {
    pub fn new() -> Self {
        Self {}
    }
}


impl Shader for DepthShader {
    fn shade(&self, fragments: &FragmentBuffer) -> Vec<Color> {
        fragments
            .z_buffer
            .iter()
            .map(|&z| {
                let v = ((z + 0.3) * 255.0) as u8;
                Color::rgba(v, v, v, 100u8)
            })
            .collect()
    }
}
