use crate::pipeline::rasterizer::FragmentBuffer;
use super::{Color, Shader};
pub struct DepthShader;

impl Shader for DepthShader {
    fn shade(fragments: &FragmentBuffer) -> Vec<Color> {
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
