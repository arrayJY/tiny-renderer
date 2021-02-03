use crate::pipeline::geometry::{model::Model, world::World};
use crate::test::about_equal;
use crate::{algebra::vector::Vector4f, *};
use std::f32::consts::PI;

fn get_test_world() -> World {
    let mut world = World::new();
    let mut model = Model::from_obj("box.obj");
    let model = model.remove(0);
    world.insert("box", model);
    world
}

#[test]
fn world_translate() {
    let mut world = get_test_world();
    world.translate("box", (-1.0, -1.0, 1.0));
    assert_eq!(
        world.model_in_world("box").unwrap().vertexs()[0],
        vector4f!(0.0, 0.0, 0.0, 1.0)
    )
}

#[test]
fn world_scale() {
    let mut world = get_test_world();
    world.scale("box", (1.0, 0.5, 2.0));
    assert_eq!(
        world.model_in_world("box").unwrap().vertexs()[0],
        vector4f!(1.0, 0.5, -2.0, 1.0)
    )
}

#[test]
fn world_rotate_around_x() -> Result<(), String> {
    let mut world = get_test_world();
    world.rotate_around_x("box", PI / 2.0);
    about_equal(
        &world.model_in_world("box").unwrap().vertexs()[0],
        &vector4f!(1.0, 1.0, 1.0, 1.0),
    )
}

#[test]
fn world_rotate_around_y() -> Result<(), String> {
    let mut world = get_test_world();
    world.rotate_around_y("box", PI / 2.0);
    about_equal(
        &world.model_in_world("box").unwrap().vertexs()[0],
        &vector4f!(-1.0, 1.0, -1.0, 1.0),
    )
}

#[test]
fn world_rotate_around_z() -> Result<(), String> {
    let mut world = get_test_world();
    world.rotate_around_z("box", PI / 2.0);
    about_equal(
        &world.model_in_world("box").unwrap().vertexs()[0],
        &vector4f!(-1.0, 1.0, -1.0, 1.0),
    )
}
