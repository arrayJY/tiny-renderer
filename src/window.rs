mod platform;

use platform::{Platform, WindowsPlatform};
use std::f32::consts::PI;

use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window as WinitWindow,
    window::WindowBuilder,
};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::renderer::Renderer;

#[allow(dead_code)]
pub struct Window {
    event_loop: EventLoop<()>,
    private_window: WinitWindow,
    platform: Platform,
}

fn render_and_redraw(platform: &Platform, renderer: &Renderer, width: usize, height: usize) {
    platform.write_buffer(&renderer.render(width, height));
    platform.redraw();
}

#[allow(dead_code)]
impl Window {
    pub fn new(width: usize, height: usize) -> Window {
        let event_loop = EventLoop::new();
        let private_window = WindowBuilder::new().build(&event_loop).unwrap();
        private_window.set_inner_size(LogicalSize::new(width as u32, height as u32));
        private_window.set_resizable(false);
        private_window.set_title("Viewer");

        let PhysicalSize { width, height } = private_window.inner_size();

        let platform = match private_window.raw_window_handle() {
            RawWindowHandle::Windows(handle) => {
                Platform::Windows(WindowsPlatform::new(handle, width, height))
            }
            _ => panic!("Unsupported platform."),
        };

        Window {
            private_window,
            event_loop,
            platform,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        let PhysicalSize { width, height } = self.private_window.inner_size();
        (width as usize, height as usize)
    }

    pub fn write_buffer(&self, buffer: &[u8]) {
        self.platform.write_buffer(buffer)
    }

    pub fn run(self, mut renderer: Renderer) {
        let (width, height) = self.size();
        self.write_buffer(&renderer.render(width, height));

        let window = self.private_window;
        let event_loop = self.event_loop;
        let platform = self.platform;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::RedrawRequested(_) => {
                    platform.redraw();
                }
                //Rotate camera
                Event::DeviceEvent {
                    event:
                        DeviceEvent::Key(KeyboardInput {
                            virtual_keycode,
                            state,
                            ..
                        }),
                    ..
                } if state == ElementState::Pressed => {
                    if let Some(keycode) = virtual_keycode {
                        match keycode {
                            VirtualKeyCode::A => {
                                renderer.yaw_camera(-PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::D => {
                                renderer.yaw_camera(PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::W => {
                                renderer.pitch_camera(PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::S => {
                                renderer.pitch_camera(-PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::J => {
                                renderer.yaw_light(-PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::L => {
                                renderer.yaw_light(PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::I => {
                                renderer.pitch_light(PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::K => {
                                renderer.pitch_light(-PI / 180.0);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::Up => {
                                renderer.zoom_camera(0.1);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            VirtualKeyCode::Down => {
                                renderer.zoom_camera(-0.1);
                                render_and_redraw(&platform, &renderer, width, height)
                            }
                            _ => {}
                        }
                    }
                }
                _ => (),
            }
        });
    }
}
