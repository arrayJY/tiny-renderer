use crate::algebra::matrix::Matrix4f;
pub struct Modeling {
    pub transform_matrix: Matrix4f,
}

#[allow(dead_code)]
impl Modeling {
    pub fn new() -> Self {
        Modeling {
            transform_matrix: Matrix4f::unit(),
        }
    }

    pub fn modeling_martix(&self) -> &Matrix4f {
        &self.transform_matrix
    }

    pub fn translate(self, (x, y, z): (f32, f32, f32)) -> Self {
        self.transform(&Matrix4f::translation_matrix(x, y, z))
    }

    pub fn scale(self, (sx, sy, sz): (f32, f32, f32)) -> Self {
        self.transform(&Matrix4f::scale_matrix(sx, sy, sz))
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
            "x" => Matrix4f::rotate_around_x_matrix(angle),
            "y" => Matrix4f::rotate_around_y_matrix(angle),
            "z" => Matrix4f::rotate_around_z_matrix(angle),
            _ => panic!("Rotate around unexpected axis."),
        };
        self.transform(&rotate_matrix)
    }

    fn transform(mut self, matrix: &Matrix4f) -> Self {
        self.transform_matrix = matrix * &self.transform_matrix;
        self
    }
}
