use crate::algebra::{matrix::Matrix4f, vector::Vector4f};
use crate::{
    pipeline::{
        camera::Camera,
        model::Model,
        rasterizer::Rasterizer,
        transformation::{modeling::Modeling, Transformation},
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

    pub fn render(self) {
        let (width, height) = (self.width, self.height);
        let window = Window::new(width, height);
        window.run(self);
    }

    pub fn bitmap_buffer(&self, width: usize, height: usize) -> Vec<u8>{
        let origin_model = self.model.as_ref().unwrap();
        let camera = self.camera.as_ref().unwrap();

        let mut model = origin_model.clone();

        model.transform(&Transformation::view_matrix(camera));
        model.transform(&Transformation::perspective_projection_transform(camera));
        model.transform(&Transformation::viewport_transform(
            width as f32 / 2.0,
            height as f32 / 2.0,
        ));
        //Move to screen center.
        model.transform(
            Modeling::new()
                .translate((width as f32 / 4.0, height as f32 / 4.0, 0.0))
                .modeling_martix(),
        );

        let triangles = model.triangles();

        let mut rasterizer = Rasterizer::new(width, height).triangles(triangles);
        rasterizer.rasterize();

        let size = width * height;
        let mut frame_buffer_bitmap = Vec::with_capacity(size * 4);

        rasterizer.frame_buffer.iter().for_each(|c| {
            frame_buffer_bitmap.push(c.b);
            frame_buffer_bitmap.push(c.g);
            frame_buffer_bitmap.push(c.r);
            frame_buffer_bitmap.push(c.a);
        });

        frame_buffer_bitmap
    }


    pub fn rotate_camera(&mut self, angle: f32) {
        let camera = self.camera.as_ref().unwrap();
        let e = &camera.eye_position;
        let p=
            Matrix4f::rotate_around_y_matrix(angle) * vector4f!(e.x(), e.y(), e.z(), 1.0);
        let mut g=  &vector4f!(0.0, 0.0, 0.0, 1.0) - &p;
        g.normalize();
        let p= vector3f!(p.x(),p.y(), p.z());
        let g= vector3f!(g.x(),g.y(), g.z());
        let u= vector3f!(g.x(),-g.y(), g.z());


        let new_camera = camera.clone().eye_position(p).gaze_direct(g).up_direct(u);
        self.camera = Some(new_camera);
    }
}
