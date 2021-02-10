use crate::{
    algebra::{vector::Vector3f},
};
pub struct Camera {
    pub gaze_direct: Vector3f,
    pub up_direct: Vector3f,
    pub eye_position: Vector3f,
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
}