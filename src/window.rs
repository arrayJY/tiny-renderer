use minifb::{Key, Window, WindowOptions};

pub struct FramebufferWindow {
    private_window: Window,
}

use crate::renderer::Renderer;
use std::f32::consts::PI;

impl FramebufferWindow {
    pub fn new(width: usize, height: usize) -> FramebufferWindow {
        let mut window =
            Window::new("TinyRenderer", width, height, WindowOptions::default()).unwrap();
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        FramebufferWindow {
            private_window: window,
        }
    }
    pub fn run(&mut self, mut renderer: Renderer) {
        let (width, height) = self.private_window.get_size();
        let mut buffer = renderer.render(width, height);
        while self.private_window.is_open() && !self.private_window.is_key_down(Key::Escape) {
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            self.private_window
                .update_with_buffer(&buffer, width, height)
                .unwrap();
            self.private_window
                .get_keys()
                .iter()
                .for_each(|key| match key {
                    Key::D => {
                        renderer.yaw_camera(PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::A => {
                        renderer.yaw_camera(-PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::W => {
                        renderer.pitch_camera(PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::S => {
                        renderer.pitch_camera(-PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::J => {
                        renderer.yaw_light(-PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::L => {
                        renderer.yaw_light(PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::I => {
                        renderer.pitch_light(PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::K => {
                        renderer.pitch_light(-PI / 180.0);
                        buffer = renderer.render(width, height);
                    }
                    Key::Up => {
                        renderer.zoom_camera(0.1);
                        buffer = renderer.render(width, height);
                    }
                    Key::Down => {
                        renderer.zoom_camera(-0.1);
                        buffer = renderer.render(width, height);
                    }
                    _ => (),
                });
        }
    }
}
