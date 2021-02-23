use super::FragmentShader;
use crate::pipeline::{model::Triangle, texture::Texture};
use crate::Color;

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

#[allow(unused_variables)]
impl FragmentShader for TextureShader {
    fn shade(&self, triangle: &Triangle, (alpha, beta, gamma): (f32, f32, f32), z: f32) -> Color {
        let &(u0, v0) = triangle.vertexs[0].texture_coordinate.as_ref().unwrap();
        let &(u1, v1) = triangle.vertexs[1].texture_coordinate.as_ref().unwrap();
        let &(u2, v2) = triangle.vertexs[2].texture_coordinate.as_ref().unwrap();

        let (mut u, mut v) = (
            u0 * alpha + u1 * beta + u2 * gamma,
            v0 * alpha + v1 * beta + v2 * gamma,
        );
        if u > 1.0 {
            u = 1.0
        } else if u < 0.0 {
            u = 0.0;
        }
        if v > 1.0 {
            v = 1.0
        } else if v < 0.0 {
            v = 0.0;
        }
        self.texture.get(u, v)
    }
}
