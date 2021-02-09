use crate::{
    algebra::vector::{Vector3f, Vector4f},
    pipeline::geometry::{model::Model, view::Camera},
    *,
};
#[test]
fn view_transformation() {
    let camera = Camera::new()
        .up_direct(vector3f!(0.0, 1.0, 0.0))
        .gaze_direct(vector3f!(0.0, 0.0, 1.0))
        .eye_position(vector3f!(1.0, 1.0, 1.0));
    let view_transform_matrix = camera.view_matrix();
    let mut model = Model::from_obj("box.obj");
    // assert_eq!(model.len(), 1);
    let mut model = model.remove(0);
    model.transform(&view_transform_matrix);

    let first_vertex = model.vertexs_mut().remove(0);
    //(1.0, 1.0, -1.0) -> (0.0, 0.0, -2.0) -> (0.0, 0.0, 2.0)
    assert_eq!(first_vertex, vector4f!(0.0, 0.0, 2.0, 1.0));
}
