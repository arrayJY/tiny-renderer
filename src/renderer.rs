use pipeline::model::Triangle;

use crate::{
    algebra::{
        matrix::Matrix4f,
        vector::{Vector3f, Vector4f},
    },
    pipeline::{
        fragment_shader::{make_shader, FragmentShader},
        light::Light,
        model::Vertex,
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

    pub fn yaw_light(&mut self, angle: f32) {
        let light = self.light.as_mut().unwrap();
        let axis = vector3f!(0.0, 1.0, 0.0);
        light.position = rotate_around_axis(&light.position, &axis, angle);
        self.shader.as_mut().unwrap().update_light(light);
    }

    pub fn pitch_light(&mut self, angle: f32) {
        let light = self.light.as_mut().unwrap();
        let p = &light.position;
        let axis = vector3f!(p.z(), 0.0, -p.x()).normalized();
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
            let model = model.clone();
            mvp_viewport_transform(model, camera, width, height)
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn mvp_viewport_transform(
    mut model: Model,
    camera: &Camera,
    width: usize,
    height: usize,
) -> Vec<Triangle> {
    // No modeling transformation for now
    let view = Transformation::view_matrix(camera);
    let projection = Transformation::perspective_projection_transform(camera);
    let viewport = Transformation::viewport_transform(width as f32, height as f32);

    transform_models_vertexs(&mut model.vertexs, &view);
    transform_models_vertexs(&mut model.vertexs, &projection);
    // let mut vertexs = homogeneous_clip(model, camera);
    let vertexs = complete_homogeneous_clip(model);
    let mut vertexs = back_face_cull(vertexs);
    triangles_w_reciprocal(&mut vertexs);
    transform_models_vertexs(&mut vertexs, &viewport);
    homogeneous_division(&mut vertexs);
    primitive_assembly(vertexs)
}

#[derive(Debug, Clone, Copy)]
enum Plane {
    W,
    Left,
    Right,
    Top,
    Bottom,
    Near,
    Far,
}

fn inside_plane(plane: Plane, p: &Vector4f) -> bool {
    match plane {
        Plane::W => p.w() < -1.0e-5,
        Plane::Left => p.x() > p.w(),
        Plane::Right => p.x() < -p.w(),
        Plane::Top => p.y() < -p.w(),
        Plane::Bottom => p.y() > p.w(),
        Plane::Near => p.z() > p.w(),
        Plane::Far => p.z() < -p.w(),
    }
}

fn get_interest_radio(plane: Plane, prev: &Vector4f, curr: &Vector4f) -> f32 {
    let pw = prev.w();
    let cw = curr.w();
    match plane {
        Plane::W => (pw + -1.0e-5) / (pw - cw),
        Plane::Left => (pw - prev.x()) / ((pw - prev.x()) - (cw - curr.x())),
        Plane::Right => (pw + prev.x()) / ((pw + prev.x()) - (cw + curr.x())),
        Plane::Top => (pw + prev.y()) / ((pw + prev.y()) - (cw + curr.y())),
        Plane::Bottom => (pw - prev.y()) / ((pw - prev.y()) - (cw - curr.y())),
        Plane::Near => (pw - prev.z()) / ((pw - prev.z()) - (cw - curr.z())),
        Plane::Far => (pw + prev.z()) / ((pw + prev.z()) - (cw + curr.z())),
    }
}

fn interpolate_vector4f(v1: &Vector4f, v2: &Vector4f, t: f32) -> Vector4f {
    v1 + &((v2 - v1) * t)
}
fn interpolate_vector3f(v1: &Vector3f, v2: &Vector3f, t: f32) -> Vector3f {
    v1 + &((v2 - v1) * t)
}

macro_rules! interpolate_option {
    ($v1: expr, $v2: expr, $t: expr) => {{
        if $v1.is_some() && $v2.is_some() {
            let v1 = $v1.as_ref().unwrap();
            let v2 = $v2.as_ref().unwrap();
            Some(v1 + &((v2 - v1) * $t))
        } else {
            None
        }
    }};
}

macro_rules! interpolate_option_pair {
    ($v1: expr, $v2: expr, $t: expr) => {{
        if $v1.is_some() && $v2.is_some() {
            let (v11, v12) = $v1.as_ref().unwrap();
            let (v21, v22) = $v2.as_ref().unwrap();
            Some((v11 + (v21 - v11) * $t, v12 + (v22 - v12) * $t))
        } else {
            None
        }
    }};
}

fn interpolate_vertex(v1: &Vertex, v2: &Vertex, t: f32) -> Vertex {
    Vertex {
        position: interpolate_vector4f(&v1.position, &v2.position, t),
        world_position: interpolate_option!(&v1.world_position, &v2.world_position, t),
        normal: interpolate_option!(&v1.normal, &v2.normal, t),
        color: {
            if v1.color.is_some() && v2.color.is_some() {
                let v1_color = v1.color.as_ref().unwrap();
                let v1_color = vector3f!(v1_color.r as f32, v1_color.g as f32, v1_color.b as f32);
                let v2_color = v2.color.as_ref().unwrap();
                let v2_color = vector3f!(v2_color.r as f32, v2_color.g as f32, v2_color.b as f32);
                let v = interpolate_vector3f(&v1_color, &v2_color, t);
                Some(Color::rgb(v.x() as u8, v.y() as u8, v.z() as u8))
            } else {
                None
            }
        },
        w_reciprocal: interpolate_option!(&v1.w_reciprocal, &v2.w_reciprocal, t),
        texture_coordinate: interpolate_option_pair!(
            &v1.texture_coordinate,
            &v2.texture_coordinate,
            t
        ),
    }
}

fn clip_plane(plane: Plane, vertexs: Vec<Vertex>) -> Vec<Vertex> {
    let current_vertexs = vertexs.iter();
    let previous_vertexs = vertexs.iter().cycle().skip(vertexs.len() - 1);
    let mut vertexs = Vec::<Vertex>::with_capacity(vertexs.len() + 1);

    for (curr, prev) in current_vertexs.zip(previous_vertexs) {
        let prev_inside = inside_plane(plane, &prev.position);
        let current_inside = inside_plane(plane, &curr.position);

        if prev_inside != current_inside {
            let radio = get_interest_radio(plane, &prev.position, &curr.position);
            vertexs.push(interpolate_vertex(prev, curr, radio));
        }

        if current_inside {
            vertexs.push(curr.clone());
        }
    }
    vertexs
}

fn complete_homogeneous_clip(model: Model) -> Vec<Vertex> {
    let vertex_groups = model
        .indices
        .iter()
        .map(|is| is.iter().map(|&i| &model.vertexs[i as usize]));

    let mut new_vertexs = Vec::with_capacity(model.vertexs.len());
    vertex_groups.for_each(|vertexs| {
        let vertexs = vertexs.cloned().collect::<Vec<_>>();
        let vertexs = clip_plane(Plane::W, vertexs);
        let vertexs = clip_plane(Plane::Left, vertexs);
        let vertexs = clip_plane(Plane::Right, vertexs);
        let vertexs = clip_plane(Plane::Top, vertexs);
        let vertexs = clip_plane(Plane::Bottom, vertexs);
        let vertexs = clip_plane(Plane::Near, vertexs);
        let vertexs = clip_plane(Plane::Far, vertexs);

        let pb_iter = vertexs.iter().skip(1);
        let pc_iter = vertexs.iter().skip(2);

        for (pb, pc) in pb_iter.zip(pc_iter) {
            let pa = &vertexs[0];
            new_vertexs.push(pa.clone());
            new_vertexs.push(pb.clone());
            new_vertexs.push(pc.clone());
        }
    });
    new_vertexs
}

fn primitive_assembly(vertexs: Vec<Vertex>) -> Vec<Triangle> {
    vertexs
        .chunks(3)
        .map(|vertexs| Triangle {
            vertexs: vertexs.to_vec(),
        })
        .collect()
}

fn back_face_cull(vertexs: Vec<Vertex>) -> Vec<Vertex> {
    vertexs
        .chunks(3)
        .filter(|&vertexs| {
            let p0 = Vector3f::from_vec4f(&vertexs[0].position);
            let p1 = Vector3f::from_vec4f(&vertexs[1].position);
            let p2 = Vector3f::from_vec4f(&vertexs[2].position);
            let l1 = &p1 - &p0;
            let l2 = &p2 - &p0;
            let e = vector3f!(0.0, 0.0, 0.0);
            let n = l1.cross(&l2).normalized();

            const ZERO: f32 = 0.05; // If it's 0.0, some visible surfaces will be culled.
            n.dot(&(&e - &p0)) < ZERO || n.dot(&(&e - &p1)) < ZERO || n.dot(&(&e - &p2)) < ZERO
            // u.cross(&v).normalized().dot(&n) < 0.0
        })
        .flatten()
        .cloned()
        .collect()
}

fn transform_models_vertexs(vertexs: &mut [Vertex], transform_matrix: &Matrix4f) {
    vertexs.iter_mut().for_each(|v| {
        v.position = transform_matrix * &v.position;
    })
}

fn triangles_w_reciprocal(vertexs: &mut [Vertex]) {
    vertexs.iter_mut().for_each(|v| {
        v.w_reciprocal = Some(1.0 / v.position.w());
    })
}

fn homogeneous_division(vertexs: &mut [Vertex]) {
    vertexs.iter_mut().for_each(|v| {
        v.position = &v.position / v.position.w();
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
