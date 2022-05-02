use crate::{
    algebra::vector_new::vector3,
    pipeline::{
        model::Model,
        ray_tracing::data_structure::bvh::{BVHNode, BVHTree},
    },
    renderer::triangulated_models_and_triangles,
    Color,
};
use rand::Rng;

#[test]
pub fn bvh() {
    let path = "static/cornell-box.obj";
    let models = Model::from_obj(path);
    let (_, triangles) = triangulated_models_and_triangles(&models, 400.0);
    println!("total: {:?}", triangles.len());
    let bvh = BVHTree::from_triangles(&triangles);
    let root = Some(Box::new(bvh.root.clone()));
    print_node(&root, 0);
}

fn print_node(node: &Option<Box<BVHNode>>, d: usize) {
    if let Some(node) = node {
        for _ in 0..d {
            print!("--");
        }
        println!("{:?}", node.bounding_box);
        print_node(&node.l, d + 1);
        print_node(&node.r, d + 1);
    }
}

#[test]
pub fn test_ray() {
    let path = "static/cornell-box.obj";
    let models = Model::from_obj(path);
    let (_, triangles) = triangulated_models_and_triangles(&models, 400.0);
    for t in triangles {
        if t.vertexs.len() != 3 {
            println!("{:?}", t.vertexs.len())
        }
    }
}

#[test]
fn rand() {
    use rand::thread_rng;
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
    println!("{:?}", thread_rng().gen_range(0.0f32..1.0));
}

#[test]
pub fn color() {
    let v = vector3([1.8, 1.5, 1.5]);
    let c = Color::from(&v);
    println!("{:?}", c);
}

#[test]
pub fn ray_range() {
    let width = 400usize;
    let height = 400usize;
    let pixel_iter = (0..width).flat_map(move |a| (0..height).map(move |b| (a, b)));
    let mut max_x = f32::MIN;
    let mut min_x = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_y = f32::MAX;

    for (x, y) in pixel_iter {
        let (x, y) = (x as f32, y as f32);
        let x = 2.0 * (x) / width as f32 - 1.0;
        let y = 1.0 - 2.0 * (y) / height as f32;
        max_x = max_x.max(x);
        min_x = min_x.min(x);
        max_y = max_y.max(y);
        min_y = min_y.min(y);
    }

    println!("{} {} {} {}", max_x, min_x, max_y, min_y);
}
