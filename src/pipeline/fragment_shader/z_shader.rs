use super::{FragmentShader, ShaderFunc};
use crate::Color;

pub struct ZShader;

impl FragmentShader for ZShader {
    fn shader(&self) -> ShaderFunc {
        Box::new(|_, _, z| &Color::rgb(255, 255, 255) * z)
    }
}
