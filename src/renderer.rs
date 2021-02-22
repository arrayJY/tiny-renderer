use pipeline::{model::Triangle, rasterizer::FragmentBuffer, shader::Color};

use crate::algebra::{
    matrix::Matrix4f,
    vector::{Vector3f, Vector4f},
};
use crate::{
    pipeline::{
        camera::Camera, model::Model, rasterizer::Rasterizer, shader::all_shaders,
        transformation::Transformation,
    },
    window::Window,
    *,
};

#[allow(dead_code)]
pub struct Renderer {
    pub models: Option<Vec<Model>>,
    pub camera: Option<Camera>,
    pub window: Option<Window>,
    pub width: usize,
    pub height: usize,
    pub shaders: Vec<fn(&FragmentBuffer) -> Vec<Color>>,
    pub shader_index: usize,
}

#[allow(dead_code)]
impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            models: None,
            camera: None,
            window: None,
            width,
            height,
            shaders: Vec::new(),
            shader_index: 0,
        }
    }

    pub fn models(mut self, models: Vec<Model>) -> Self {
        self.models = Some(models);
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn shaders(mut self, shaders: Vec<fn(&FragmentBuffer) -> Vec<Color>>) -> Self {
        self.shaders = shaders;
        self
    }

    pub fn run(self) {
        let (width, height) = (self.width, self.height);
        let window = Window::new(width, height);
        window.run(self);
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let camera = self.camera.as_ref().unwrap();
        let models = self.models.as_ref().unwrap();
        let triangles = triangles_from_models(models, camera, width, height);

        let mut rasterizer = Rasterizer::new(width, height).triangles(triangles);
        rasterizer.rasterize();

        assert!(!self.shaders.is_empty());

        let shader = self.shaders[self.shader_index];
        let frame_buffer = shader(&rasterizer.fragment_buffer);
        let bitmap = bitmap_from_framebuffer(&frame_buffer, width, height);
        bitmap
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

impl Default for Renderer {
    fn default() -> Self {
        Self {
            models: None,
            camera: Some(Camera::default()),
            window: None,
            width: 800,
            height: 800,
            shaders: all_shaders(),
            shader_index: 0,
        }
    }
}

/** Some functions **/

fn rotate_around_axis(v: &Vector3f, axis: &Vector3f, angle: f32) -> Vector3f {
    v * angle.cos() + axis.cross(v) * angle.sin() + axis * axis.dot(v) * (1.0 - angle.cos())
}

fn triangles_from_models(
    models: &[Model],
    camera: &Camera,
    width: usize,
    height: usize,
) -> Vec<Triangle> {
    models
        .iter()
        .map(|m| {
            let mut model = m.clone();
            mvp_viewport_transform(&mut model, camera, width, height);
            model.triangles()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn mvp_viewport_transform(model: &mut Model, camera: &Camera, width: usize, height: usize) {
    model.transform(&Transformation::view_matrix(camera));
    model.transform(&Transformation::perspective_projection_transform(camera));
    model.transform(&Transformation::viewport_transform(
        width as f32,
        height as f32,
    ));
}

fn bitmap_from_framebuffer(frame_buffer: &[Color], width: usize, height: usize) -> Vec<u8> {
    let mut frame_buffer_bitmap = Vec::with_capacity(width * height * 4);
    frame_buffer
        .iter()
        .enumerate()
        .step_by(width)
        .rev()
        .map(|(i, ..)| &frame_buffer[i..i + width])
        .for_each(|line| {
            line.iter().for_each(|c| {
                frame_buffer_bitmap.push(c.b);
                frame_buffer_bitmap.push(c.g);
                frame_buffer_bitmap.push(c.r);
                frame_buffer_bitmap.push(c.a);
            })
        });

    frame_buffer_bitmap
}
