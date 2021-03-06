use crate::{
    algebra::vector::Vector3f,
    pipeline::{camera::Camera, light::Light, model::Triangle, texture::Texture},
    renderer::Renderer,
    Color, *,
};

use super::FragmentShader;

pub struct PhongShader {
    pub eye_position: Vector3f,
    pub light: Light,
    pub texture: Option<Texture>,
}

impl PhongShader {
    pub fn color_shader(renderer: &Renderer) -> Self {
        let e = &renderer.camera.as_ref().unwrap().eye_position;
        let l = renderer.light.as_ref().unwrap();
        Self {
            eye_position: e.clone(),
            light: l.clone(),
            texture: None,
        }
    }
    pub fn texture_shader(renderer: &Renderer, path: &str) -> Self {
        let mut shader = PhongShader::color_shader(renderer);
        let jpg_texture = Texture::from_path(&format!("{}.jpg", path));
        let png_texture = Texture::from_path(&format!("{}.png", path));
        let texture = jpg_texture.or(png_texture);
        shader.texture = match texture {
            Ok(texture) => Some(texture),
            Err(err) => {
                println!("Load texture failed: {}", err);
                println!("Use phong-color shader.");
                None
            }
        };
        shader
    }
}

macro_rules! interpolate {
    ($triangle: tt, $attr: ident; $barycenter: expr) => {{
        let (alpha, beta, gamma) = $barycenter;
        let v1 = $triangle.vertexs[0].$attr.as_ref().unwrap();
        let v2 = $triangle.vertexs[1].$attr.as_ref().unwrap();
        let v3 = $triangle.vertexs[2].$attr.as_ref().unwrap();
        v1 * alpha + v2 * beta + v3 * gamma
    }};
}

impl FragmentShader for PhongShader {
    fn shade(&self, triangle: &Triangle, barycenter: (f32, f32, f32), _: f32) -> Color {
        let Color { r, g, b, .. } = if let Some(texture) = &self.texture {
            let &(u0, v0) = triangle.vertexs[0].texture_coordinate.as_ref().unwrap();
            let &(u1, v1) = triangle.vertexs[1].texture_coordinate.as_ref().unwrap();
            let &(u2, v2) = triangle.vertexs[2].texture_coordinate.as_ref().unwrap();
            let (alpha, beta, gamma) = barycenter;
            let (u, v) = (
                u0 * alpha + u1 * beta + u2 * gamma,
                v0 * alpha + v1 * beta + v2 * gamma,
            );
            texture.get(u, v)
        } else {
            interpolate!(triangle, color; barycenter)
        };
        let position = Vector3f::from_vec4f(&interpolate!(triangle, world_position; barycenter));
        let normal = Vector3f::from_vec4f(&interpolate!(triangle, normal; barycenter));

        let ambient_light_intensity = vector3f!(10.0, 10.0, 10.0);
        let ka = vector3f!(0.005, 0.005, 0.005);
        let kd = vector3f!(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let ks = vector3f!(0.8, 0.8, 0.8);
        let p = 150;

        let eye_positon = &self.eye_position;
        let Light {
            position: light_position,
            intensity,
        } = &self.light;

        let n = normal.normalized();
        let l = (light_position - &position).normalized();
        let v = (eye_positon - &position).normalized();
        let h = (&l + &v).normalized();

        let r = (light_position - &position).norm();

        let ambient = ka.cwise_product(&ambient_light_intensity);
        let diffuse = kd * (*intensity / (r * r) * max(0.0, n.dot(&l)));
        let specular = ks * (*intensity / (r * r) * max(0.0, n.dot(&h)).powi(p));

        let result_color = (ambient + diffuse + specular) * 255.0;

        let (r, g, b) = (
            result_color.x() as u8,
            result_color.y() as u8,
            result_color.z() as u8,
        );
        Color::rgb(r, g, b)
    }
    fn update_camera(&mut self, camera: &Camera) {
        self.eye_position = camera.eye_position.clone();
    }
    fn update_light(&mut self, light: &Light) {
        self.light = light.clone();
    }
}

fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
