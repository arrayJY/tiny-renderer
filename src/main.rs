mod algebra;
mod pipeline;
mod renderer;
mod window;

#[cfg(test)]
mod test;

use std::f32::consts::PI;

use algebra::vector::Vector3f;
use pipeline::{camera::Camera, model::Model};
use renderer::Renderer;
fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = WIDTH;
    let model = Model::from_obj("box.obj").remove(0);
    let camera = Camera::new()
        .eye_position(vector3f!(3.0, 3.0, 3.0))
        .gaze_direct(vector3f!(-1.0, -1.0, -1.0))
        .up_direct(vector3f!(-1.0, 1.0, -1.0))
        .eye_fov(PI / 2.0)
        .aspect_ratio(WIDTH as f32 / HEIGHT as f32)
        .near(1.5)
        .far(20.0);

    Renderer::new(WIDTH, HEIGHT)
        .model(model)
        .camera(camera)
        .render();
}
