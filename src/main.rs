use std::env;
use tiny_renderer::{
    pipeline::model::Model, ray_tracing::path_tracing::RayTracer, renderer::Renderer,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let shader_name = &args[1];
    let path = &args[2];
    let model_path = format!("{}.obj", path);

    if shader_name == "pbr" {
        const DEFAULT_SPP: usize = 8;
        let spp = args
            .get(3)
            .map_or(DEFAULT_SPP, |s| s.parse::<usize>().unwrap_or(DEFAULT_SPP));
        RayTracer::render(&model_path, spp);
    } else {
        // let shader = make_shader(shader_name, path);
        Renderer::default()
            .models(Model::from_obj(&model_path))
            .shader(shader_name, path)
            .run();
    }
}
