use minifb::{Key, Window, WindowOptions};

use crate::pipeline::ray_tracing::{
    pbr_shading::RayTracer,
};

pub struct PBRWindow {
    window: Window,
}

impl PBRWindow {
    pub fn new(width: usize, height: usize) -> Self {
        let window = Window::new("TinyRenderer", width, height, WindowOptions::default()).unwrap();
        Self { window }
    }

    pub fn run(&mut self, ray_tracer: RayTracer) {
        let (width, height) = self.window.get_size();
        let buffer = ray_tracer.frame_buffer();
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&buffer, width, height)
                .unwrap()
        }
    }
}
