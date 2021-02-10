use modeling::Modeling;
use pipeline::camera::Camera;

use crate::{*, algebra::vector::Vector3f, algebra::matrix::Matrix4f};

pub mod modeling;

pub struct Transformation;
#[allow(dead_code)]
impl Transformation{
    pub fn modeling_matrix(modeling: Modeling) -> Matrix4f {
        modeling.transform_matrix
    }

    pub fn view_matrix(camera: &Camera) -> Matrix4f {
        let e = &camera.eye_position;
        let g = &camera.gaze_direct;
        let t = &camera.up_direct;

        let w: Vector3f = -(g / g.norm());
        let u: Vector3f = t.cross(&w) / t.cross(&w).norm();
        let v: Vector3f = w.cross(&u);

        let translate_matrix = matrix4f!(
            1.0, 0.0, 0.0, -e.x(),
            0.0, 1.0, 0.0, -e.y(),
            0.0, 0.0, 1.0, -e.z(),
            0.0, 0.0, 0.0, 1.0
        );

        let rotate_matrix = matrix4f!(
            u.x(), u.y(), u.z(), 0.0,
            v.x(), v.y(), v.z(), 0.0,
            w.x(), w.y(), w.z(), 0.0,
              0.0,   0.0,   0.0, 1.0
        );

        rotate_matrix * translate_matrix  
    }

}