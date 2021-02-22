use super::Shader;
use crate::pipeline::rasterizer::FragmentBuffer;
use crate::Color;

#[allow(dead_code)]
pub struct VertexShader;

impl Shader for VertexShader {
    fn shade(fragments: &FragmentBuffer) -> Vec<Color> {
        fragments
            .color_buffer
            .iter()
            .map(|color| {
                color
                    .clone()
                    .unwrap_or_else(|| Color::rgba(255, 255, 255, 100))
            })
            .collect()
    }
}
