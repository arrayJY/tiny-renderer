use pipeline::model::Triangle;

use crate::algebra::{
    matrix::Matrix4f,
    vector::{Vector3f, Vector4f},
};
use crate::{
    pipeline::{
        camera::Camera,
        fragment_shader::{all_shaders, ShaderFunc},
        model::Model,
        rasterizer::Rasterizer,
        transformation::Transformation,
    },
    window::Window,
    Color, *,
};

#[allow(dead_code)]
pub struct Renderer {
    pub models: Option<Vec<Model>>,
    pub camera: Option<Camera>,
    pub window: Option<Window>,
    pub width: usize,
    pub height: usize,
    pub shaders: Vec<fn() -> ShaderFunc>,
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

    pub fn shaders(mut self, shaders: Vec<fn() -> ShaderFunc>) -> Self {
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

        //Transformation
        let triangles = triangles_from_models(models, camera, width, height);

        //Rasterization
        let mut rasterizer = Rasterizer::new(width, height).triangles(triangles);
        let shader = self.shaders[self.shader_index]();
        let frame_buffer = rasterizer.rasterize(&shader);

        //Shading
        assert!(!self.shaders.is_empty());
        //let frame_buffer = shader(&rasterizer.);
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
        .map(|model| {
            let mut triangles = model.clone().triangles();
            mvp_viewport_transform(&mut triangles, camera, width, height);
            triangles
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn mvp_viewport_transform(
    triangles: &mut [Triangle],
    camera: &Camera,
    width: usize,
    height: usize,
) {
    transform_triangles(triangles, &Transformation::view_matrix(camera));
    normalize_triangles_vertexs(triangles);
    transform_triangles(
        triangles,
        &Transformation::perspective_projection_transform(camera),
    );
    //TODO: Clip
    normalize_triangles_vertexs(triangles);
    transform_triangles(
        triangles,
        &Transformation::viewport_transform(width as f32, height as f32),
    );
}

fn transform_triangles(triangles: &mut [Triangle], transform_matrix: &Matrix4f) {
    triangles.iter_mut().for_each(|t| {
        t.vertexs.iter_mut().for_each(|v| {
            v.position = transform_matrix * &v.position;
        })
    })
}

fn normalize_triangles_vertexs(triangles: &mut [Triangle]) {
    triangles.iter_mut().for_each(|t| {
        t.vertexs.iter_mut().for_each(|v| {
            v.position = &v.position / v.position.w();
        })
    })
}

fn bitmap_from_framebuffer(frame_buffer: &[Option<Color>], width: usize, height: usize) -> Vec<u8> {
    let mut frame_buffer_bitmap = Vec::with_capacity(width * height * 4);
    //Background
    let background = [255u8, 255, 255, 100];
    frame_buffer
        .iter()
        .enumerate()
        .step_by(width)
        .rev()
        .map(|(i, ..)| &frame_buffer[i..i + width])
        .for_each(|line| {
            line.iter().for_each(|c| {
                if let Some(c) = c {
                    frame_buffer_bitmap.push(c.b);
                    frame_buffer_bitmap.push(c.g);
                    frame_buffer_bitmap.push(c.r);
                    frame_buffer_bitmap.push(c.a);
                } else {
                    background.iter().for_each(|&c| {
                        frame_buffer_bitmap.push(c);
                    })
                }
            })
        });

    frame_buffer_bitmap
}
