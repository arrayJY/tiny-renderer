use std::env;
use tiny_renderer::{pipeline::model::Model, renderer::Renderer};

fn main() {
    let args: Vec<String> = env::args().collect();
    let shader_name = &args[1];
    let path = &args[2];
    let model_path = format!("{}.obj", path);
    // let shader = make_shader(shader_name, path);
    Renderer::default()
        .models(Model::from_obj(&model_path))
        .shader(shader_name, path)
        .run();
}
