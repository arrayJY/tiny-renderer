use crate::{
    pipeline::{
        camera::Camera,
        model::Model,
        rasterizer::Rasterizer,
        transformation::{modeling::Modeling, Transformation},
    },
    window::Window,
};
use std::time::{Instant};

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
        let width = self.width;
        let height = self.height;

        let window = Window::new(width, height);
        let (width, height) = window.size();

        let mut model = self.model.unwrap();
        let camera = self.camera.unwrap();

        let start = Instant::now();
        model.transform(&Transformation::view_matrix(&camera));
        model.transform(&Transformation::perspective_projection_transform(&camera));
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
        let duration = start.elapsed();
        println!("Transformation cast: {:?}.", duration);

        let start = Instant::now();
        let triangles = model.triangles();

        let mut rasterizer = Rasterizer::new(width, height).triangles(triangles);
        rasterizer.rasterize();
        let duration = start.elapsed();
        println!("Rasterization cast: {:?}.", duration);

        let size = width * height;
        let mut frame_buffer_bitmap = Vec::with_capacity(size * 4);

        let start = Instant::now();
        rasterizer.frame_buffer.iter().for_each(|c| {
            frame_buffer_bitmap.push(c.b);
            frame_buffer_bitmap.push(c.g);
            frame_buffer_bitmap.push(c.r);
            frame_buffer_bitmap.push(c.a);
        });

        let duration = start.elapsed();
        window.write_buffer(&frame_buffer_bitmap[..]);
        println!("Copy bitmap to screen cast: {:?}", duration);
        window.run();
    }
}
