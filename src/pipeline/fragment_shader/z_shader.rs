use super::{FragmentShader, FragmentShaderPayload};
use crate::{
    pipeline::{
        camera::Camera,
        light::Light,
    },
    Color,
};

pub struct ZShader;

#[allow(unused_variables)]
impl FragmentShader for ZShader {
    fn shade(&self, FragmentShaderPayload { z, .. }: &FragmentShaderPayload) -> Color {
        &Color::rgb(255, 255, 255) * (z * 5.0 - 4.0)
    }
    fn update_camera(&mut self, _: &Camera) {}
    fn update_light(&mut self, _: &Light) {}
}
