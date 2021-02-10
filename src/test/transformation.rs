use pipeline::transformation::modeling::Modeling;
use test::about_equal;
use std::f32::consts::PI;

use crate::{
    algebra::vector::{Vector3f, Vector4f},
    pipeline::{model::Model, camera::Camera, transformation::Transformation},
    *,
};

fn load_model() -> Model{
    let mut model = Model::from_obj("box.obj");
    model.remove(0)
}

#[test]
fn modeling_transformation() -> Result<(), String>{
    let modeling = Modeling::new() 
    .translate((1.0, 1.0, 1.0))
    .scale((0.5, 2.0, 1.0))
    .rotate_around_x(PI / 2.0);

    let mut model = load_model();
    let modeling_matrix = Transformation::modeling_matrix(modeling);
    model.transform(&modeling_matrix);


    let first_vertex = model.vertexs_mut().remove(0);
    //(1.0, 1.0, -1.0) -> (2.0, 2.0, 0.0) -> (1.0, 4.0, 0.0) -> (1.0, 0.0, 4.0)
    about_equal(&first_vertex, &vector4f!(1.0, 0.0, 4.0, 1.0))
}


#[test]
fn view_transformation() {
    let camera = Camera::new()
        .up_direct(vector3f!(0.0, 1.0, 0.0))
        .gaze_direct(vector3f!(0.0, 0.0, 1.0))
        .eye_position(vector3f!(1.0, 1.0, 1.0));
    let view_transform_matrix = Transformation::view_matrix(&camera);
    let mut model = load_model();
    // assert_eq!(model.len(), 1);
    model.transform(&view_transform_matrix);

    let first_vertex = model.vertexs_mut().remove(0);
    //(1.0, 1.0, -1.0) -> (0.0, 0.0, -2.0) -> (0.0, 0.0, 2.0)
    assert_eq!(first_vertex, vector4f!(0.0, 0.0, 2.0, 1.0));
}