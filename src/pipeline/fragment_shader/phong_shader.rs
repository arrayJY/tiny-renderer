use crate::{
    algebra::vector_new::{vector3, Vector3},
    pipeline::{
        camera::Camera,
        light::Light,
        model::{Triangle, TriangulatedModel},
        texture::Texture,
    },
    renderer::Renderer,
    Color, *,
};

use super::FragmentShader;

pub struct PhongShader {
    pub eye_position: Vector3,
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

const DEFAULT_KD: Vector3 = vector3([0.5, 0.5, 0.5]);
const DEFAULT_KA: Vector3 = vector3([1.0, 1.0, 1.0]);
const DEFAULT_KS: Vector3 = vector3([0.8, 0.8, 0.8]);
const AMBIENT_INTENSITY: Vector3 = vector3([0.2, 0.2, 0.2]);

impl FragmentShader for PhongShader {
    fn shade(
        &self,
        model: &TriangulatedModel,
        triangle: &Triangle,
        barycenter: (f32, f32, f32),
        _: f32,
    ) -> Color {
        let material = model.material.as_ref().and_then(|m| m.phong_material());

        let kd = if let Some(texture) = &self.texture {
            let (u, v) = interpolate_uv!(triangle, texture_coordinate; barycenter);
            let color = texture.get(u, v);
            vector3([color.r as f32, color.g as f32, color.b as f32]) / 255.0
        } else {
            material.map_or(DEFAULT_KD, |m| m.diffuse_color.clone())
        };
        let position = Vector3::from(&interpolate_triangle!(triangle, world_position; barycenter));
        let normal = Vector3::from(&interpolate!(triangle, normal; barycenter));
        let ka = material.map_or(DEFAULT_KA, |m| m.ambient_color.clone());
        let ks = material.map_or(DEFAULT_KS, |m| m.specular_color.clone());
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

        let ambient = ka.cwise_product(&AMBIENT_INTENSITY);
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
