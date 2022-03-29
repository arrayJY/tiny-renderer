use modeling::Modeling;
use pipeline::camera::Camera;

use crate::{
    algebra::matrix_new::{matrix4, MatrixNew4},
    algebra::vector_new::VectorNew3,
    *,
};

pub mod modeling;

pub struct Transformation;
#[allow(dead_code)]
impl Transformation {
    pub fn modeling_matrix(modeling: Modeling) -> MatrixNew4 {
        modeling.transform_matrix
    }

    pub fn view_matrix(camera: &Camera) -> MatrixNew4 {
        let e = &camera.eye_position;
        let g = &camera.gaze_direct;
        let t = &camera.up_direct;

        let w: VectorNew3 = -(g / g.norm());
        let u: VectorNew3 = t.cross(&w) / t.cross(&w).norm();
        let v: VectorNew3 = w.cross(&u);

        let translate_matrix = MatrixNew4::translation_matrix(-e.x(), -e.y(), -e.z());

        let rotate_matrix = matrix4([
            [u.x(), u.y(), u.z(), 0.0],
            [v.x(), v.y(), v.z(), 0.0],
            [w.x(), w.y(), w.z(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        rotate_matrix * translate_matrix
    }

    pub fn orthogonal_projection_transform(camera: &Camera) -> MatrixNew4 {
        let n = -camera.near;
        let f = -camera.far;

        let t = -n * (camera.eye_fov / 2.0).tan();
        let b = -t;

        let r = t * camera.aspect_ratio;
        let l = -r;

        let translate_matrix =
            MatrixNew4::translation_matrix(-(r + l) / 2.0, -(t + b) / 2.0, -(n + f) / 2.0);

        let scale_matrix = MatrixNew4::scale_matrix(2.0 / (r - l), 2.0 / (t - b), 2.0 / (n - f));

        scale_matrix * translate_matrix
    }

    pub fn perspective_projection_transform(camera: &Camera) -> MatrixNew4 {
        let n = -camera.near;
        let f = -camera.far;
        let persp_to_ortho = matrix4([
            [n, 0.0, 0.0, 0.0],
            [0.0, n, 0.0, 0.0],
            [0.0, 0.0, n + f, -f * n],
            [0.0, 0.0, 1.0, 0.0],
        ]);
        let ortho = Transformation::orthogonal_projection_transform(camera);

        ortho * persp_to_ortho
    }

    pub fn viewport_transform(width: f32, height: f32) -> MatrixNew4 {
        matrix4([
            [width / 2.0, 0.0, 0.0, width / 2.0],
            [0.0, height / 2.0, 0.0, height / 2.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
