use std::env;
use tiny_renderer::{
    pipeline::{model::Model, ray_tracing::pbr_shading::RayTracer},
    renderer::Renderer,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let shader_name = &args[1];
    let path = &args[2];
    let model_path = format!("{}.obj", path);

    if shader_name == "pbr" {
        const DEFAULT_SSP: usize = 8;
        let bounce = args
            .get(3)
            .map_or(DEFAULT_SSP, |s| s.parse::<usize>().unwrap_or(DEFAULT_SSP));
        RayTracer::render(&model_path, bounce);
    } else {
        // let shader = make_shader(shader_name, path);
        Renderer::default()
            .models(Model::from_obj(&model_path))
            .shader(shader_name, path)
            .run();
    }
}
