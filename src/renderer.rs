use pipeline::shader::{Shader};

use crate::algebra::{matrix::Matrix4f, vector::Vector4f};
use crate::{
    pipeline::{
        camera::Camera, model::Model, rasterizer::Rasterizer, transformation::Transformation,
    },
    window::Window,
    *,
};

#[allow(dead_code)]
pub struct Renderer {
    pub model: Option<Model>,
    pub camera: Option<Camera>,
    pub window: Option<Window>,
    pub rasterizer: Option<Rasterizer>,
    pub width: usize,
    pub height: usize,
    pub shaders: Vec<Box<dyn Shader>>,
    pub shader_index: usize,
}

fn rotate_around_axis(v: &Vector3f, axis: &Vector3f, angle: f32) -> Vector3f {
    v * angle.cos() + axis.cross(v) * angle.sin() + axis * axis.dot(v) * (1.0 - angle.cos())
}

#[allow(dead_code)]
impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            model: None,
            camera: None,
            window: None,
            rasterizer: None,
            width,
            height,
            shaders: Vec::new(),
            shader_index: 0,
        }
    }

    pub fn model(mut self, model: Model) -> Self {
        self.model = Some(model);
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn shaders(mut self, shaders: Vec<Box<dyn Shader>>) -> Self {
        self.shaders = shaders;
        self
    }

    pub fn render(self) {
        let (width, height) = (self.width, self.height);
        let window = Window::new(width, height);
        window.run(self);
    }

    pub fn bitmap_buffer(&self, width: usize, height: usize) -> Vec<u8> {
        let origin_model = self.model.as_ref().unwrap();
        let camera = self.camera.as_ref().unwrap();

        let mut model = origin_model.clone();

        model.transform(&Transformation::view_matrix(camera));
        model.transform(&Transformation::perspective_projection_transform(camera));
        model.transform(&Transformation::viewport_transform(
            width as f32,
            height as f32,
        ));
        let triangles = model
            .triangles()
            .iter()
            .filter_map(|t| {
                //Simple clip
                if t.points.iter().any(|v| {
                    v.x() >= 0.0 && v.x() <= width as f32 && v.y() >= 0.0 && v.y() <= height as f32
                }) {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut rasterizer = Rasterizer::new(width, height).triangles(triangles);
        rasterizer.rasterize();

        let size = width * height;
        let mut frame_buffer_bitmap = Vec::with_capacity(size * 4);

        assert!(!self.shaders.is_empty());

        let shader = self.shaders[self.shader_index].as_ref();
        let frame_buffer = shader.shade(&rasterizer.fragment_buffer);

        frame_buffer.iter().rev().for_each(|c| {
            frame_buffer_bitmap.push(c.b);
            frame_buffer_bitmap.push(c.g);
            frame_buffer_bitmap.push(c.r);
            frame_buffer_bitmap.push(c.a);
        });

        frame_buffer_bitmap
    }

    pub fn next_shader(&mut self) {
        if self.shaders.len() <= self.shader_index + 1 {
            self.shader_index = 0;
        } else {
            self.shader_index += 1;
        }
    }

    pub fn yaw_camera(&mut self, angle: f32) {
        let camera = self.camera.as_ref().unwrap();
        let e = &camera.eye_position;
        let p = Matrix4f::rotate_around_y_matrix(angle) * Vector4f::from_vec3f_point(&e);
        let p = Vector3f::from_vec4f(&p);

        let y_axis = vector3f!(0.0, 1.0, 0.0);
        let g = rotate_around_axis(&camera.gaze_direct, &y_axis, angle);
        let u = rotate_around_axis(&camera.up_direct, &y_axis, angle);

        let new_camera = camera.clone().eye_position(p).gaze_direct(g).up_direct(u);
        self.camera = Some(new_camera);
    }

    pub fn pitch_camera(&mut self, angle: f32) {
        let camera = self.camera.as_ref().unwrap();
        let mut axis = camera.up_direct.cross(&camera.gaze_direct);
        axis.normalize();
        let e = rotate_around_axis(&camera.eye_position, &axis, angle);
        let g = rotate_around_axis(&camera.gaze_direct, &axis, angle);
        let u = rotate_around_axis(&camera.up_direct, &axis, angle);
        let new_camera = camera.clone().eye_position(e).gaze_direct(g).up_direct(u);
        self.camera = Some(new_camera);
    }

    pub fn zoom_camera(&mut self, length: f32) {
        let camera = self.camera.as_ref().unwrap();
        let g = Vector4f::from_vec3f_vector(&camera.gaze_direct);
        let p: Vector4f = Vector4f::from_vec3f_point(&camera.eye_position) + (g * length);
        let p = Vector3f::from_vec4f(&p);
        let new_camera = camera.clone().eye_position(p);
        self.camera = Some(new_camera);
    }
}
