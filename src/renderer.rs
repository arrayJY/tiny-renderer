use pipeline::model::Triangle;

use crate::{
    algebra::{
        matrix::Matrix4f,
        vector::{Vector3f, Vector4f},
    },
    pipeline::{
        fragment_shader::{make_shader, FragmentShader},
        light::Light,
    },
};
use crate::{
    pipeline::{
        camera::Camera, model::Model, rasterizer::Rasterizer, transformation::Transformation,
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
    pub shader: Option<Box<dyn FragmentShader>>,
    pub height: usize,
    pub light: Option<Light>,
}

#[allow(dead_code)]
impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            models: None,
            camera: None,
            window: None,
            shader: None,
            light: None,
            width,
            height,
        }
    }

    pub fn models(mut self, models: Vec<Model>) -> Self {
        self.models = Some(models);
        self
    }

    pub fn shader(mut self, name: &str, path: &str) -> Self {
        let shader = make_shader(name, path, &self);
        self.shader = shader;
        self
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn run(self) {
        let (width, height) = (self.width, self.height);
        let window = Window::new(width, height);
        window.run(self);
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<u8> {
        let models = self
            .models
            .as_ref()
            .unwrap_or_else(|| panic!("No models found."));
        let camera = self
            .camera
            .as_ref()
            .unwrap_or_else(|| panic!("No camera found."));
        let fragment_shader = self
            .shader
            .as_ref()
            .unwrap_or_else(|| panic!("No fragment shader found."));

        //Transformation
        let triangles = triangles_from_models(models, camera, width, height);

        //Rasterization && Shading
        let mut rasterizer = Rasterizer::new(width, height).triangles(triangles);
        let frame_buffer = rasterizer.rasterize(fragment_shader);

        //Generate Bitmap
        let bitmap = bitmap_from_framebuffer(&frame_buffer, width, height);
        bitmap
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
        self.shader.as_mut().unwrap().update_camera(&new_camera);
        self.camera = Some(new_camera);
    }

    pub fn pitch_camera(&mut self, angle: f32) {
        let camera = self.camera.as_ref().unwrap();
        let axis = camera.up_direct.cross(&camera.gaze_direct).normalized();
        let e = rotate_around_axis(&camera.eye_position, &axis, angle);
        let g = rotate_around_axis(&camera.gaze_direct, &axis, angle);
        let u = rotate_around_axis(&camera.up_direct, &axis, angle);
        let new_camera = camera.clone().eye_position(e).gaze_direct(g).up_direct(u);
        self.shader.as_mut().unwrap().update_camera(&new_camera);
        self.camera = Some(new_camera);
    }

    pub fn yaw_light(&mut self, angle: f32)  {
        let light = self.light.as_mut().unwrap();
        let axis = vector3f!(0.0, 1.0, 0.0);
        light.position = rotate_around_axis(&light.position, &axis, angle);
        self.shader.as_mut().unwrap().update_light(light);
    }

    pub fn pitch_light(&mut self, angle: f32)  {
        let light = self.light.as_mut().unwrap();
        let p = &light.position;
        let axis = vector3f!(p.z(), 0.0 , -p.x()).normalized();
        light.position = rotate_around_axis(&light.position, &axis, angle);
        self.shader.as_mut().unwrap().update_light(light);
    }

    pub fn zoom_camera(&mut self, length: f32) {
        let camera = self.camera.as_ref().unwrap();
        let g = Vector4f::from_vec3f_vector(&camera.gaze_direct);
        let p: Vector4f = Vector4f::from_vec3f_point(&camera.eye_position) + (g * length);
        let p = Vector3f::from_vec4f(&p);
        let new_camera = camera.clone().eye_position(p);
        self.shader.as_mut().unwrap().update_camera(&new_camera);
        self.camera = Some(new_camera);
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            models: None,
            camera: Some(Camera::default()),
            window: None,
            shader: None,
            light: Some(Light::default()),
            width: 800,
            height: 800,
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
            let triangles = model.clone().triangles();
            mvp_viewport_transform(triangles, camera, width, height)
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn mvp_viewport_transform(
    mut triangles: Vec<Triangle>,
    camera: &Camera,
    width: usize,
    height: usize,
) -> Vec<Triangle> {
    // No modeling transformation for now
    let view = Transformation::view_matrix(camera);
    let projection = Transformation::perspective_projection_transform(camera);
    let viewport = Transformation::viewport_transform(width as f32, height as f32);

    transform_triangles_vertexs(&mut triangles, &view);
    transform_triangles_vertexs(&mut triangles, &projection);

    triangles = homogeneous_clip(triangles, camera);

    triangles_w_reciprocal(&mut triangles);
    transform_triangles_vertexs(&mut triangles, &viewport);
    homogeneous_division(&mut triangles);
    triangles
}

// Simple clip
fn homogeneous_clip(triangles: Vec<Triangle>, camera: &Camera) -> Vec<Triangle> {
    triangles
        .into_iter()
        .filter(|t| {
            !t.vertexs.iter().any(|v| {
                let x = v.position.x();
                let y = v.position.y();
                let z = v.position.z();
                let w = v.position.w();
                let n = -camera.near;
                let f = -camera.far;
                (x < w || x > -w) || (y < w || y > -w) || (z < w || z > -w) || (n < w || w < f)
            })
        })
        .collect()
}

fn transform_triangles_vertexs(triangles: &mut [Triangle], transform_matrix: &Matrix4f) {
    triangles.iter_mut().for_each(|t| {
        t.vertexs.iter_mut().for_each(|v| {
            v.position = transform_matrix * &v.position;
        })
    })
}

fn triangles_w_reciprocal(triangles: &mut [Triangle]) {
    triangles.iter_mut().for_each(|t| {
        t.vertexs.iter_mut().for_each(|v| {
            v.w_reciprocal = Some(1.0 / v.position.w());
        })
    })
}


fn homogeneous_division(triangles: &mut [Triangle]) {
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
