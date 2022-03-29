use super::{matrix4, Matrix4};

impl Matrix4 {
    pub fn translation_matrix(x: f32, y: f32, z: f32) -> Matrix4 {
        matrix4([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale_matrix(sx: f32, sy: f32, sz: f32) -> Matrix4 {
        matrix4([
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn rotate_around_x_matrix(angle: f32) -> Matrix4 {
        let sina = angle.sin();
        let cosa = angle.cos();
        matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cosa, -sina, 0.0],
            [0.0, sina, cosa, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn rotate_around_y_matrix(angle: f32) -> Matrix4 {
        let sina = angle.sin();
        let cosa = angle.cos();
        matrix4([
            [cosa, 0.0, sina, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sina, 0.0, cosa, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn rotate_around_z_matrix(angle: f32) -> Matrix4 {
        let sina = angle.sin();
        let cosa = angle.cos();
        matrix4([
            [cosa, -sina, 0.0, 0.0],
            [sina, cosa, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
