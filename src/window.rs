mod platform;
use platform::{Platform, WindowsPlatform};

use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window as WinitWindow,
    window::WindowBuilder,
};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[allow(dead_code)]
pub struct Window {
    event_loop: EventLoop<()>,
    private_window: WinitWindow,
    platform: Platform,
}

#[allow(dead_code)]
impl Window {
    pub fn new() -> Window {
        const WIDTH: u32 = 400;
        const HEIGHT: u32 = WIDTH;

        let event_loop = EventLoop::new();
        let private_window = WindowBuilder::new().build(&event_loop).unwrap();
        private_window.set_inner_size(LogicalSize::new(WIDTH, HEIGHT));
        private_window.set_resizable(false);

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

    pub fn write_buffer(&self, buffer: &[u8]) {
        self.platform.write_buffer(buffer)
    }

    pub fn run(self) {
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
                _ => (),
            }
        });
    }
}
