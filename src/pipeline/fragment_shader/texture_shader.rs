use super::FragmentShader;
use crate::pipeline::model::TriangulatedModel;
use crate::pipeline::{model::Triangle, texture::Texture};
use crate::{
    interpolate_uv,
    pipeline::{camera::Camera, light::Light},
    Color,
};

pub struct TextureShader {
    texture: Texture,
}

impl TextureShader {
    pub fn new(path: &str) -> Self {
        //Try jpg
        if let Ok(texture) = Texture::from_path(&format!("{}.jpg", path)) {
            TextureShader { texture }
        } else {
            //Try png
            if let Ok(texture) = Texture::from_path(&format!("{}.png", path)) {
                TextureShader { texture }
            } else {
                panic!("Cannot find texture file `{0}.jpg` or `{0}.png`.", path)
            }
        }
    }
}

impl FragmentShader for TextureShader {
    fn shade(
        &self,
        _: &TriangulatedModel,
        triangle: &Triangle,
        barycenter: (f32, f32, f32),
        _: f32,
    ) -> Color {
        let (u, v) = interpolate_uv!(triangle, texture_coordinate; barycenter);
        self.texture.get(u, v)
    }
    fn update_camera(&mut self, _: &Camera) {}
    fn update_light(&mut self, _: &Light) {}
}
