use super::{Color, Shader};
use crate::blend_color;
use crate::pipeline::rasterizer::FragmentBuffer;

#[allow(dead_code)]
pub struct VertexShader;

impl Shader for VertexShader {
    fn shade(fragments: &FragmentBuffer) -> Vec<Color> {
        let c1 = Color::rgba(255, 102, 153, 100);
        let c2 = Color::rgba(103, 153, 255, 100);
        let c3 = Color::rgba(153, 255, 102, 100);
        let white = Color::rgba(255, 255, 255, 100);
        fragments
            .barycenter_buffer
            .iter()
            .map(|barycenter| {
                let mut color = white.clone();
                if let Some(barycenter) = barycenter {
                    let (alpha, beta, gamma) = *barycenter;
                    color = blend_color!((&c1, alpha), (&c2, beta), (&c3, gamma))
                }
                color
            })
            .collect()
    }
}
