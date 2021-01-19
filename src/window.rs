use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window as WinitWindow,
    window::WindowBuilder,
};

#[allow(dead_code)]
pub struct Window {
    event_loop: EventLoop<()>,
    private_window: WinitWindow,
}

#[allow(dead_code)]
impl Window {
    pub fn new() -> Window {
        let event_loop = EventLoop::new();
        Window {
            private_window: WindowBuilder::new().build(&event_loop).unwrap(),
            event_loop,
        }
    }

    pub fn run(self) {
        let window = self.private_window;
        let event_loop = self.event_loop;
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
