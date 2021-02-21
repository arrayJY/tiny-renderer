use pipeline::model::Model;
use renderer::Renderer;
use tiny_renderer::*;
fn main() {
    Renderer::default()
        .models(Model::from_obj("examples/multi_models.obj"))
        .run();
}
