use modeling::Modeling;
use pipeline::camera::Camera;

use crate::{algebra::matrix::Matrix4f, algebra::vector::Vector3f, *};

pub mod modeling;

pub struct Transformation;
#[allow(dead_code)]
impl Transformation {
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
            0.0, 0.0, 0.0, 1.0
        );

        rotate_matrix * translate_matrix
    }

    pub fn orthogonal_projection_transform(camera: &Camera) -> Matrix4f {
        let n = -camera.near;
        let f = -camera.far;

        let t = -n * (camera.eye_fov / 2.0).tan();
        let b = -t;

        let r = t * camera.aspect_ratio;
        let l = -r;

        let translate_matrix =
            Matrix4f::translation_matrix(-(r + l) / 2.0, -(t + b) / 2.0, -(n + f) / 2.0);

        let scale_matrix = Matrix4f::scale_matrix(2.0 / (r - l), 2.0 / (t - b), 2.0 / (n - f));

        scale_matrix * translate_matrix
    }

    pub fn perspective_projection_transform(camera: &Camera) -> Matrix4f {
        let n = -camera.near;
        let f = -camera.far;
        let persp_to_ortho = matrix4f!(
              n, 0.0, 0.0,  0.0,
            0.0,   n, 0.0,  0.0,
            0.0, 0.0, n+f, -f*n,
            0.0, 0.0, 1.0,  0.0
        );
        let ortho = Transformation::orthogonal_projection_transform(camera);


        ortho * persp_to_ortho
    }

    pub fn viewport_transform(width: f32, height: f32) -> Matrix4f {
        matrix4f!(
            width/2.0,        0.0, 0.0, width/2.0,
                  0.0, height/2.0, 0.0, height/2.0,
                  0.0,        0.0, 1.0, 0.0,
                  0.0,        0.0, 0.0, 1.0
        )
    }
}
