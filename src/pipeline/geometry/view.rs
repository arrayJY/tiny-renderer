use crate::{
    algebra::{
        matrix::Matrix4f,
        vector::{Vector3f},
    },
    *,
};
pub struct Camera {
    gaze_direct: Vector3f,
    up_direct: Vector3f,
    eye_position: Vector3f,
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Self {
        Camera {
            gaze_direct: Vector3f::new(),
            up_direct: Vector3f::new(),
            eye_position: Vector3f::new(),
        }
    }

    pub fn gaze_direct(mut self, g: Vector3f) -> Self {
        self.gaze_direct = g;
        self
    }
    pub fn up_direct(mut self, u: Vector3f) -> Self {
        self.up_direct = u;
        self
    }
    pub fn eye_position(mut self, e: Vector3f) -> Self {
        self.eye_position = e;
        self
    }

    pub fn view_matrix(&self) -> Matrix4f {
        let e = &self.eye_position;
        let g = &self.gaze_direct;
        let t = &self.up_direct;

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
