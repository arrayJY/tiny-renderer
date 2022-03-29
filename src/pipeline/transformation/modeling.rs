use crate::algebra::matrix_new::Matrix4;
pub struct Modeling {
    pub transform_matrix: Matrix4,
}

#[allow(dead_code)]
impl Modeling {
    pub fn new() -> Self {
        Modeling {
            transform_matrix: Matrix4::unit(),
        }
    }

    pub fn modeling_martix(&self) -> &Matrix4 {
        &self.transform_matrix
    }

    pub fn translate(self, (x, y, z): (f32, f32, f32)) -> Self {
        self.transform(&Matrix4::translation_matrix(x, y, z))
    }

    pub fn scale(self, (sx, sy, sz): (f32, f32, f32)) -> Self {
        self.transform(&Matrix4::scale_matrix(sx, sy, sz))
    }

    pub fn rotate_around_x(self, angle: f32) -> Self {
        self.rotate_around_axis(angle, "x")
    }

    pub fn rotate_around_y(self, angle: f32) -> Self {
        self.rotate_around_axis(angle, "y")
    }

    pub fn rotate_around_z(self, angle: f32) -> Self {
        self.rotate_around_axis(angle, "z")
    }

    fn rotate_around_axis(self, angle: f32, axis: &str) -> Self {
        let rotate_matrix = match axis {
            "x" => Matrix4::rotate_around_x_matrix(angle),
            "y" => Matrix4::rotate_around_y_matrix(angle),
            "z" => Matrix4::rotate_around_z_matrix(angle),
            _ => panic!("Rotate around unexpected axis."),
        };
        self.transform(&rotate_matrix)
    }

    fn transform(mut self, matrix: &Matrix4) -> Self {
        self.transform_matrix = matrix * &self.transform_matrix;
        self
    }
}
